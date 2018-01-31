use common::db::AsyncPgPool;
use common::db::query::*;
use common::http;
use common::http::FutureHandled;
use common::http::ServerResponse;
use common::http::header::UserID;

use futures::{Future, Stream};
use futures::future::ok;

use hyper;
use hyper::{Method, Request, Response};
use hyper::server::Service;

use json;

use std::rc::Rc;

pub struct LoginHandler {
    db_conn: Rc<AsyncPgPool>,
}

impl LoginHandler {
    pub fn new(db_conn: Rc<AsyncPgPool>) -> Self {
        LoginHandler { db_conn }
    }
}

impl Service for LoginHandler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureHandled;

    fn call(&self, mut req: Request) -> Self::Future {
        unimplemented!()
    }
}
