use hyper::Request;

use futures::prelude::*;

use http::middleware::{
    Middleware,
    Transition,
    TransitionResult
};

/// `/health` service returning OK status if microservice is running (obviously)
#[derive(Debug, Copy, Clone)]
pub struct Health;

impl Middleware for Health {
    #[async(boxed)]
    fn handle(self: Box<Self>, _req: Request) -> TransitionResult {
        Ok(Transition::success(HealthStatus::ok()))
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