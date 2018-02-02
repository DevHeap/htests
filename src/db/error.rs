#![allow(unused_doc_comment)]
#![allow(missing_docs)]
//! Database error_chain

error_chain! {
    foreign_links {
        R2D2(::r2d2::Error);
        Diesel(::postgres::Error);
    }
}
