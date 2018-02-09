//! Request authentication proxy middleware

use token::AsyncTokenVerifier;
use http::ApiError;

use http::error::ErrorKind;
use http::header::UserID;

use http::middleware::{
    Middleware,
    Transition,
    TransitionResult,
};

use hyper::Request;
use hyper::header::{Authorization, Bearer};

use futures::prelude::*;
use std::rc::Rc;

/// Authenticator Service factory with "persistent" state
///
/// For usage example please refer to one of already implemented microservices
#[derive(Clone)]
pub struct Authenticator {
    auth: Rc<AsyncTokenVerifier>,
}

impl Authenticator {
    /// Create a new AuthenticatorService factory with persistent state
    pub fn new() -> Self {
        info!("Created Authenticator (Service Factory)");
        Authenticator {
            auth: Rc::new(AsyncTokenVerifier::new()),
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
    #[async(boxed)]
    fn handle(self: Box<Self>, mut req: Request) -> TransitionResult {
        trace!("accepted {} request for {}", req.method(), req.uri());
        trace!("headers: {:?}", req.headers());

        // Extract Token from headers
        let token = match Self::extract_token(&req) {
            Ok(token) => token.to_owned(),
            Err(error) => return Ok(Transition::errored(error)),
        };

        let auth_result = await!(self.auth.authenticate(token));

        match auth_result {
            Err(e) => {
                debug!("attempted unathorized access to {}", req.path());
                Ok(Transition::errored(e))
            },
            Ok(token) => {
                debug!("authorized request from user {}", token.user_id());

                // Set UserID header
                let uid = token.user_id().to_owned();
                req.headers_mut().set(UserID(uid));

                Ok(Transition::Request(req))
            },
        }
    }
}