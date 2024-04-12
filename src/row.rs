use std::sync::Arc;

use sqlx_core::{column::ColumnIndex, ext::ustr::UStr, row::Row, Error};

use crate::{
    column::D1Column,
    database::D1,
    value::{D1Value, D1ValueRef},
};

pub struct D1Row {
    values: Box<[D1Value]>,
    columns: Arc<Vec<D1Column>>,
    column_names: Arc<Vec<UStr>>,
}

impl Row for D1Row {
    type Database = D1;

    fn columns(&self) -> &[D1Column] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<D1ValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        Ok(D1ValueRef::value(&self.values[index]))
    }
}
