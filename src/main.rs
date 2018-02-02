#![feature(proc_macro, conservative_impl_trait, generators, box_syntax)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate chrono;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

extern crate futures;
extern crate futures_cpupool;

#[macro_use]
extern crate log;
extern crate fern;
extern crate hyper;
extern crate reqwest;
extern crate openssl;

#[macro_use]
extern crate error_chain;

extern crate tokio_core;

/// Database model, schema and convenience traits
pub mod db;
/// Tokens decode and verification routines
pub mod token;
/// Everything for Hyper and Http servers
#[macro_use]
pub mod http;

mod login;

use db::AsyncPgPool;

use futures::Stream;
use http::service::Authenticator;
use http::service::Health;

use hyper::server::Http;
use hyper::server::NewService;

use std::rc::Rc;
use std::sync::mpsc::channel;
use std::thread;

use tokio_core::net::TcpListener;
use tokio_core::reactor;

use login::LoginHandler;

use http::middleware::Chains;

// @TODO move to a shared library, implement log.toml config file
fn init_logger() -> Result<(), log::SetLoggerError> {
    let (tx, rx) = channel();
    thread::spawn(move || while let Ok(msg) = rx.recv() {
        print!("{}", msg);
    });

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LogLevelFilter::Warn)
        .level_for("service", log::LogLevelFilter::Trace)
        .level_for("common", log::LogLevelFilter::Trace)
        .chain(tx)
        .apply()?;
    Ok(())
}

fn main() {
    init_logger().unwrap();
    info!("initialized logger");

    let addr = "0.0.0.0:7701".parse().unwrap();

    // Connection to database
    // @TODO read db uri from config file
    let db_uri = "postgres://devheap:Olb5Ind3rT@localhost/circles-dev";
    let pgpool = Rc::new(AsyncPgPool::connect(db_uri).unwrap());

    // Starting tokio event loop
    let mut core = reactor::Core::new().expect("Failed to initialize event loop");
    let handle = core.handle();

    // Authenticator for token verification and user info population in the database
    let authenticator = Authenticator::new(pgpool.clone());

    // Router to dispatch requests for concrete pathes to their handlers
    let router = router!(
        post_login:     Method::Post, "/login"      => Rc::new(LoginHandler::new(pgpool.clone())),
        restricted:     Method::Get,  "/restricted" => Rc::new(Chains::builder()
            .chain(Box::new(authenticator))
            .chain(Box::new(Health))
            .build()),
    );

    // Starting TCP server listening for incoming commections
    let listener = TcpListener::bind(&addr, &handle).unwrap();
    let server = listener.incoming().for_each(move |(sock, addr)| {
        let entry_service = router.new_service()
        // Can never happen
            .unwrap();

        // Handing TCP connections over to Hyper
        Http::new().bind_connection(&handle, sock, addr, entry_service);
        Ok(())
    });

    // Launching an event loop: unless it is spinned up, nothing happens
    core.run(server).expect("Critical server failure");
}
