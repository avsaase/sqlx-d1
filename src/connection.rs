use futures_core::future::BoxFuture;
use futures_util::future;
use sqlx_core::{connection::Connection, transaction::Transaction, Error};

use crate::{database::D1, options::D1ConnectOptions};

pub struct D1Connection;

impl Connection for D1Connection {
    type Database = D1;

    type Options = D1ConnectOptions;

    fn close(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        Transaction::begin(self)
    }

    fn shrink_buffers(&mut self) {}

    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn should_flush(&self) -> bool {
        false
    }
}
