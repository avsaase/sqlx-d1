use sqlx_core::database::{Database, HasArguments, HasStatement, HasValueRef};

use crate::{
    arguments::{D1ArgumentValue, D1Arguments},
    column::D1Column,
    connection::D1Connection,
    query_result::D1QueryResult,
    row::D1Row,
    statement::D1Statement,
    transaction::D1TransactionManager,
    type_info::D1TypeInfo,
    value::{D1Value, D1ValueRef},
};

#[derive(Debug)]
pub struct D1(worker_sys::D1Database);

impl Database for D1 {
    type Connection = D1Connection;

    type TransactionManager = D1TransactionManager;

    type Row = D1Row;

    type QueryResult = D1QueryResult;

    type Column = D1Column;

    type TypeInfo = D1TypeInfo;

    type Value = D1Value;

    const NAME: &'static str = "Cloudflare D1";

    const URL_SCHEMES: &'static [&'static str] = &["d1"];
}

impl<'q> HasStatement<'q> for D1 {
    type Database = Self;

    type Statement = D1Statement<'q>;
}

impl<'q> HasArguments<'q> for D1 {
    type Database = Self;

    type Arguments = D1Arguments<'q>;

    type ArgumentBuffer = Vec<D1ArgumentValue<'q>>;
}

impl<'r> HasValueRef<'r> for D1 {
    type Database = Self;

    type ValueRef = D1ValueRef<'r>;
}

unsafe impl Send for D1 {}
unsafe impl Sync for D1 {}
