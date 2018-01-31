#![feature(conservative_impl_trait, box_syntax, specialization)]
#![deny(missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

//! Crate of the common building blocks for the microservices
//!
//! Everything that may be used twice or more in separate microservices
//! must be placed here.

#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate futures;
extern crate futures_cpupool;

#[macro_use]
extern crate log;
extern crate hyper;
extern crate reqwest;
extern crate openssl;

#[macro_use]
extern crate error_chain;

/// Database model, schema and convenience traits
pub mod db;
/// Tokens decode and verification routines
pub mod token;

/// Everything for Hyper and Http servers
#[macro_use]
pub mod http;
