use std::rc::Rc;
use std::io;

use hyper;
use hyper::server::{Request, Response};
use hyper::server::{Service, NewService};

use http::ApiError;
use http::error::ErrorKind;
use http::ServerResponse;

use futures::Future;
use futures::future::{
    ok,
    loop_fn,
    Loop,
};

type ChainsInner = Vec<Box<CloneableMiddleware>>;
type RcChainsInner = Rc<Vec<Box<CloneableMiddleware>>>;

#[derive(Clone)]
pub struct Chains {
    chains: RcChainsInner
}

pub struct ChainsBuilder {
    chains: ChainsInner
}

impl Chains {
    pub fn builder() -> ChainsBuilder {
        ChainsBuilder { chains: vec![] }
    }

    pub fn get(&self, idx: usize) -> Option<Box<Middleware>> {
        self.chains.get(idx).map(|m| CloneableMiddleware::clone(&**m))
    }
}

impl ChainsBuilder {
    pub fn chain(mut self, chain: Box<CloneableMiddleware>) -> Self {
        self.chains.push(chain);
        self
    }

    pub fn build(self) -> Chains {
        Chains {
            chains: RcChainsInner::new(self.chains)
        }
    }
}

impl NewService for Chains {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Chains;

    fn new_service(&self) -> io::Result<Self::Instance> {
        debug!("Created midlleware::Chain instance");
        Ok(self.clone())
    }
}

impl Service for Chains {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        trace!("accepted {} request for {}", req.method(), req.uri());

        let chains = self.clone();

        type LoopResult<L, B> = Box<Future<Item = Loop<L, B>, Error = hyper::Error>>;

        let future = loop_fn((req, 0), move |(req, idx)| -> LoopResult<_, _> {
            if let Some(chain) = chains.get(idx) {
                box chain.handle(req).and_then(move |ts| {
                    match ts {
                        Transition::Request(req) => {
                            ok(Loop::Continue((req, idx + 1)))
                        },
                        Transition::Response(resp) => {
                            ok(Loop::Break(Ok(resp)))
                        }
                    }
                })
            } else {
                box ok(Loop::Break(Err(ErrorKind::UnfinishedChain)))
            }
        });

        box future.map(|result| {
            match result {
                Ok(resp) => resp,
                Err(err) => ServerResponse::from(err).into()
            }
        })
    }
}


pub type TransitionResult = Result<Transition, hyper::Error>;
pub type FutureTransition = Box<Future<Item = Transition, Error = hyper::Error>>;

pub enum Transition {
    Request(Request),
    Response(Response)
}

use serde::Serialize;

impl Transition {
    pub fn errored<E: Into<ApiError>>(e: E) -> Transition {
        Transition::Response(
            ServerResponse::from(
                e.into()
            ).into()
        )
    }

    pub fn success<D: Serialize>(d: D) -> Transition {
        Transition::Response(
            ServerResponse::Data(d).into()
        )
    }
}

pub trait Middleware {
    fn handle(self: Box<Self>, req: Request) -> FutureTransition;
}

pub trait CloneableMiddleware: Middleware {
    fn clone(&self) -> Box<Middleware>;
}

impl<T: Middleware + Clone + 'static> CloneableMiddleware for T {
    fn clone(&self) -> Box<Middleware> {
        box Clone::clone(self)
    }
}
