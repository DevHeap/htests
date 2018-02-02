//! Request authentication proxy middleware

use db::AsyncPgPool;
use token::AsyncTokenVerifier;

use token::Token;
use futures::Future;
use futures::future;
use http::ApiError;

use http::FutureHandled;
use http::HandlerFactory;
use http::ServerResponse;
use http::error::Error;
use http::error::ErrorKind;
use http::header::UserID;
use http::middleware::{Middleware, Transition, FutureTransition};

use hyper;
use hyper::{Request, Response};
use hyper::header::{Authorization, Bearer};
use hyper::server::{Service, NewService};

use std::collections::HashMap;
use std::io;
use std::rc::Rc;

/// Authenticator Service factory with "persistent" state
///
/// For usage example please refer to one of already implemented microservices
pub struct Authenticator {
    auth: Rc<AsyncTokenVerifier>,
    users_db_updater: Rc<UsersDbUpdater>,
}

impl Authenticator {
    /// Create a new AuthenticatorService factory with persistent state
    pub fn new(db: Rc<AsyncPgPool>) -> Self {
        info!("Created Authenticator (Service Factory)");
        Authenticator {
            auth: Rc::new(AsyncTokenVerifier::new()),
            users_db_updater: Rc::new(UsersDbUpdater::new(db)),
        }
    }

    fn extract_token(req: &Request) -> Result<&str, ApiError> {
        let headers = req.headers();
        let bearer: &Authorization<Bearer> = headers.get().ok_or(ApiError::from(
            ErrorKind::AuthHeaderMissing,
        ))?;

        Ok(&bearer.token)
    }
}

impl Middleware for Authenticator {
    fn handle(&self, mut req: Request) -> Box<Future<Item = Transition, Error = hyper::Error>> {
        trace!("accepted {} request for {}", req.method(), req.uri());
        trace!("headers: {:?}", req.headers());

        // Extract Token from headers
        let token = match Self::extract_token(&req) {
            Ok(token) => token.to_owned(),
            Err(error) => return box future::ok(
                Transition::Response(ServerResponse::from(error).into())
            ),
        };

        let users_db = self.users_db_updater.clone();

        // Either pass the request to the Dispatcher or return error response to a client
        let future_response = self.auth.authenticate(token).map_err(Error::from).then(
            move |auth_result| -> FutureTransition {
                match auth_result {
                    Ok(token) => {
                        debug!("authorized request from user {}", token.user_id());

                        // Set UserID header
                        let uid = token.user_id().to_owned();
                        req.headers_mut().set(UserID(uid));

                        // Update users database table and proceed to the router
                        let db_future = users_db.update_if_needed(&token).then(
                            move |res| match res {
                                Ok(..) => box future::ok(Transition::Request(req)),
                                Err(e) => box future::ok(
                                    Transition::Response(ServerResponse::from(e).into())
                                ),
                            },
                        );

                        // Pass the request to dispatcher
                        box db_future
                    }
                    Err(e) => {
                        debug!("attempted unathorized access to {}", req.path());
                        box future::ok(
                            Transition::Response(ServerResponse::from(e).into())
                        )
                    }
                }
            },
        );

        box future_response
    }
}

use chrono::NaiveDateTime;
use std::cell::RefCell;

// Cached updater preventing unneeded sql request flood
struct UsersDbUpdater {
    db: Rc<AsyncPgPool>,
    /// Mapping from user_id to token expiration time
    auth_table: RefCell<HashMap<String, NaiveDateTime>>,
}

impl UsersDbUpdater {
    fn new(db: Rc<AsyncPgPool>) -> Self {
        UsersDbUpdater {
            db,
            auth_table: RefCell::new(HashMap::new()),
        }
    }

    // Update DB only for users whose token expiration was not cached yet
    fn update_if_needed(&self, token: &Token) -> Box<Future<Item = (), Error = Error>> {
        let user_id = token.user_id();
        let exp = unimplemented!();//NaiveDateTime::from_timestamp(token.expiration_time() as i64, 0);

        if Some(&exp) != self.auth_table.borrow().get(user_id) {
            debug!(
                "no cached expiration entry for {}, adding & updating DB",
                user_id
            );
            self.auth_table.borrow_mut().insert(user_id.to_owned(), exp);
            box self.update(token)
        } else {
            debug!(
                "found cached expiration entry for {}, doing nothing",
                user_id
            );
            box future::ok(())
        }
    }

    fn update(&self, token: &Token) -> impl Future<Item = (), Error = Error> {
        use db::models::User;
        use db::query::Insert;

        let user = User::from(token);
        let user_id = user.uid.clone();

        user.insert(&self.db)
            .then(move |result| {
                match result {
                    Ok(ref rows) => {
                        debug!("successfully updated {} rows for user {}", rows, user_id)
                    }
                    Err(ref e) => error!("failed to update db for user {}: {}", user_id, e),
                }
                result.map(|_| ())
            })
            .map_err(Error::from)
    }
}
