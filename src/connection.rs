use futures_core::future::BoxFuture;
use sqlx_core::{connection::Connection, transaction::Transaction, Error};

use crate::{database::D1, options::D1ConnectOptions};

pub struct D1Connection;

impl Connection for D1Connection {
    type Database = D1;

    type Options = D1ConnectOptions;

    fn close(self) -> BoxFuture<'static, Result<(), Error>> {
        todo!()
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        todo!()
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        todo!()
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn shrink_buffers(&mut self) {
        todo!()
    }

    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        todo!()
    }

    fn should_flush(&self) -> bool {
        todo!()
    }
}
