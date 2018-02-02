
//! Data types reflecting actual database tables schema

use chrono::NaiveDateTime;
use token::Token;

/// User model for the "users" table
#[derive(Debug, Clone)]
pub struct User {
    /// User unique identifier from Google Firebase API
    pub uid: String,
    /// Username
    pub username: Option<String>,
    /// User email
    pub email: Option<String>,
    /// Token issue time (basically an authentication time)
    pub auth_time: NaiveDateTime,
    /// Token expiration time
    pub auth_until: NaiveDateTime,
}

/// Auth data changeset: issue time and expiration time
#[derive(Debug, Copy, Clone)]
pub struct UserAuthData {
    /// Token issue time (basically an authentication time)
    pub auth_time: NaiveDateTime,
    /// Token expiration time
    pub auth_until: NaiveDateTime,
}

impl<'a> From<&'a Token> for User {
    fn from(token: &'a Token) -> Self {
        // TODO
        unimplemented!()
    }
}

impl User {
    /// Get AuthData of a User
    pub fn auth_data(&self) -> UserAuthData {
        UserAuthData {
            auth_time: self.auth_time,
            auth_until: self.auth_until,
        }
    }
}
