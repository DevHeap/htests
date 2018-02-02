use futures::future::ok;
use http::FutureHandled;
use http::ServerResponse;
use hyper;
use hyper::Request;
use hyper::Response;
use hyper::server::NewService;
use hyper::server::Service;
use std::io;

use http::middleware::{
    Chains,
    Middleware,
    Transition,
    FutureTransition
};

/// `/health` service returning OK status if microservice is running (obviously)
#[derive(Debug, Copy, Clone)]
pub struct Health;

impl Middleware for Health {
    fn handle(&self, _req: Request) -> FutureTransition {
        box ok(Transition::Response(
            ServerResponse::Data(HealthStatus::ok()).into()
        ))
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
struct HealthStatus {
    status: &'static str,
}

impl HealthStatus {
    fn ok() -> Self {
        HealthStatus { status: "OK" }
    }
}