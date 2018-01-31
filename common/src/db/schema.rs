#![allow(non_snake_case, missing_docs, unused_qualifications)]

//! Diesel generated schema and tables access DSL

table! {
    users (uid) {
        uid -> Varchar,
        username -> Nullable<Varchar>,
        picture -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        auth_time -> Timestamp,
        auth_until -> Timestamp,
    }
}
