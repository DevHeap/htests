
//! Token Verifiers built with Google Keyring

use token::{Result, Error, ErrorKind};
use token::Token;

use json;
use openssl::aes::AesKey;
use reqwest;
use reqwest::StatusCode;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub type Key = AesKey;

/// TokenVerifier verifies tokens using the key
pub struct TokenVerifier {
    key: Key,
}

impl TokenVerifier {
    /// Constructs a TokenVerifier
    pub fn new(key: Key) -> Self {
        TokenVerifier { key }
    }

    /// Decode and verify a Token
    pub fn verify_token<T>(&self, token: T) -> Result<Token>
    where
        T: Into<Cow<'static, str>>,
    {
        // Token::decode(&token.into(), &self.key)
        unimplemented!()
    }
}

use futures::Future;
use futures_cpupool::CpuPool;

/// CpuPool driven token authentifier
pub struct AsyncTokenVerifier {
    cpupool: CpuPool,
    verifier: Arc<TokenVerifier>,
}

impl AsyncTokenVerifier {
    /// Constructs an AsyncTokenVerifier with a TokenVerifier
    /// and a CpuPool to run async verification tasks
    pub fn new() -> Self {
        let key = unimplemented!();
        AsyncTokenVerifier {
            cpupool: CpuPool::new_num_cpus(),
            verifier: Arc::new(TokenVerifier::new(key)),
        }
    }

    /// Asynchronously Verify JWT Token
    pub fn authenticate(&self, token: String) -> impl Future<Item = Token, Error = Error> {
        let verifier = self.verifier.clone();
        self.cpupool.spawn_fn(move || verifier.verify_token(token))
    }
}
