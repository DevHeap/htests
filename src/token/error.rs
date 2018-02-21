#![allow(unused_doc_comment)]
#![allow(missing_docs)]

use http::ApiError;
use hyper::StatusCode;

// Generate error types boilerplate

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Json(::json::error::Error);
        Hyper(::hyper::Error);
        Utf8(::std::string::FromUtf8Error);
        OpenSSL(::openssl::error::Error);
        OpenSSLStack(::openssl::error::ErrorStack);
        Reqwest(::reqwest::Error);
    }

    errors {
        TokenExpired {
            description("token expired")
            display("token expired")
        }

        DecodeFailed{
            description("token decode failed")
            display("token decode failed")
        }
    }
}

/// This function converts ErrorKind and &ErrorKind to an ErrorResponse
impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::from(e.kind())
    }
}

impl From<ErrorKind> for ApiError {
    fn from(ek: ErrorKind) -> Self {
        ApiError::from(&ek)
    }
}

impl<'a> From<&'a ErrorKind> for ApiError {
    fn from(ek: &'a ErrorKind) -> Self {
        use token::ErrorKind::*;
        match *ek {
            Io(..) |
            Hyper(..) |
            OpenSSL(..) |
            OpenSSLStack(..) |
            Reqwest(..) |
            Msg(..) => ApiError::with_status(&ek, StatusCode::InternalServerError),
            _ => ApiError::with_status(&ek, StatusCode::Unauthorized),
        }
    }
}
