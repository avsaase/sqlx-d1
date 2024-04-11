use futures_core::future::BoxFuture;
use sqlx_core::{transaction::TransactionManager, Error};

use crate::{connection::D1Connection, database::D1};

pub struct D1TransactionManager;

impl TransactionManager for D1TransactionManager {
    type Database = D1;

    fn begin(_conn: &mut D1Connection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "begin not supported",
            )))
        })
    }

    fn commit(_conn: &mut D1Connection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "commit not supported",
            )))
        })
    }

    fn rollback(_conn: &mut D1Connection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "rollback not supported",
            )))
        })
    }

    fn start_rollback(conn: &mut D1Connection) {}
}
