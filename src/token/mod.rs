//! Token decoding and verigication
//! @TODO: testspub mod error;

pub mod error;
pub mod verifier;

// Export main elements
pub use self::error::{Result, Error, ErrorKind};
pub use self::verifier::{Key, TokenVerifier, AsyncTokenVerifier};

use std::string;
use std::ops::Deref;

use json;

use chrono::prelude::*;
use chrono::{Duration, Utc};

use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::Nonce;

const LIFETIME: i64 = 864000; // 10 days in sec

/// Authorization Token
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Token {
    user_id: String,
    created: i64,
}

impl Token {
    pub fn issue(user_id: &str) -> Token {
        Token{user_id: user_id.to_string(), created: Utc::now().timestamp()}
    }

    pub fn decode(token: Vec<u8>, key: &Key) -> Result<Token> {
        let (head, sealed) = token.split_at(24);
        let nonce = Nonce::from_slice(head).unwrap();

        let open = secretbox::open(&sealed, &nonce, &key)
            .map_err(
                |_| ErrorKind::DecodeFailed
            )?;

        let result: Token = json::from_slice(&open)?;

        return Ok(result);
    }

    pub fn encode(&self, key: &Key) -> Result<Vec<u8>> {
        let serialized = json::to_string(self)?;

        let nonce  = secretbox::gen_nonce();
        let sealed = secretbox::seal(serialized.as_bytes(), &nonce, &key);

        let mut result = Vec::<u8>::new();

        result.extend(nonce.as_ref());
        result.extend(sealed);

        return Ok(result);
    }


    /// Get user unique identifier
    pub fn user_id(&self) -> &str {
        return &self.user_id
    }

    /// Check that token expired
    pub fn is_expired(&self) -> bool {
        // Verify token lifetime time

        let t_now = Utc::now();
        let t_created = Utc.timestamp(self.created, 0);
        let max_duration = Duration::seconds(LIFETIME);

        if t_now.signed_duration_since(t_created) > max_duration {
            return true;
        }

        return false;
    }
}

#[test]
fn token_ecnrypt_decrypt() {
    let key = secretbox::gen_key();

    let token = Token::issue("user_test");

    let crypted = token.encode(&key).unwrap();
    let result = Token::decode(crypted, &key).unwrap();

    assert_eq!(token, result);
}