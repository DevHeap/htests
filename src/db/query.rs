//! Convenience traits for easy querying

use db::AsyncPgPool;
use db::error::Error;
use db::models::*;

use futures_cpupool::CpuFuture;

/// Convenient trait to simplify object insertion
pub trait Insert {
    /// Insertion consumes the object Self and returns a future numbers of changed rows
    fn insert(self, pool: &AsyncPgPool) -> CpuFuture<u64, Error>;
}

impl Insert for User {
    fn insert(self, pool: &AsyncPgPool) -> CpuFuture<u64, Error> {
        use futures::future::result;

        // Insert an authentified user or, if user exists, just update
        pool.request(move |conn| {
            result(
                conn.execute(
                    "INSERT INTO users VALUES($1, $2, $3, $4)
                     ON CONFLICT DO UPDATE SET auth_time = $3, auth_until = $4", &[
                        &self.username,
                        &self.email,
                        &self.auth_time,
                        &self.auth_until
                ]).map_err(Error::from)
            )
        })
    }
}
