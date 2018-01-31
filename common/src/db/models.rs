
//! Data types reflecting actual database tables schema

use chrono::NaiveDateTime;
use db::schema::*;
use token::Token;

/// User model for the "users" table
#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "users"]
#[primary_key(uid)]
pub struct User {
    /// User unique identifier from Google Firebase API
    pub uid: String,
    /// Username
    pub username: Option<String>,
    /// Uri of userpic
    pub picture: Option<String>,
    /// User email
    pub email: Option<String>,
    /// Firebase token issue time (basically an authentication time)
    pub auth_time: NaiveDateTime,
    /// Firebase token expiration time
    pub auth_until: NaiveDateTime,
}

/// Auth data changeset: issue time and expiration time
#[derive(Debug, Copy, Clone, AsChangeset)]
#[table_name = "users"]
pub struct UserAuthData {
    /// Firebase token issue time (basically an authentication time)
    pub auth_time: NaiveDateTime,
    /// Firebase token expiration time
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
