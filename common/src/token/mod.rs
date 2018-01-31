//! Token decoding and verigication
//! @TODO: testspub mod error;

pub mod error;
pub mod verifier;

// Export main elements
pub use self::error::{Result, Error, ErrorKind};
pub use self::verifier::{Key, TokenVerifier, AsyncTokenVerifier};

use json;
use std::ops::Deref;

/// Authorization Token
#[derive(Copy, Clone, Debug)]
pub struct Token {
    // TODO: add fields
}

impl Token {
    /// Decode and verify the base64 encode JWT Token using provided Keyring
    pub fn decode(token: &str, key: &Key) -> Result<Token> {
        /* todo */
        unimplemented!()
    }

    /// Get user unique identifier
    pub fn user_id(&self) -> &str {
        unimplemented!()
    }

    fn verify(&self) -> Result<()> {
        /* todo: check token validity */
        unimplemented!()
    }
}