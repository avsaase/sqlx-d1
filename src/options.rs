use std::{str::FromStr, time::Duration};

use log::LevelFilter;
use sqlx_core::{connection::ConnectOptions, Error, Url};

use crate::connection::D1Connection;

#[derive(Debug, Clone)]
pub struct D1ConnectOptions;

impl ConnectOptions for D1ConnectOptions {
    type Connection = D1Connection;

    fn from_url(url: &Url) -> Result<Self, Error> {
        todo!()
    }

    fn connect(&self) -> futures_core::future::BoxFuture<'_, Result<Self::Connection, Error>>
    where
        Self::Connection: Sized,
    {
        todo!()
    }

    fn log_statements(self, level: LevelFilter) -> Self {
        todo!()
    }

    fn log_slow_statements(self, level: LevelFilter, duration: Duration) -> Self {
        todo!()
    }
}

impl FromStr for D1ConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
