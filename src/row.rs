use sqlx_core::{column::ColumnIndex, row::Row, Error};

use crate::{database::D1, value::D1ValueRef};

pub struct D1Row;

impl Row for D1Row {
    type Database = D1;

    fn columns(&self) -> &[<Self::Database as sqlx_core::database::Database>::Column] {
        todo!()
    }

    fn try_get_raw<I>(&self, index: I) -> Result<D1ValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        todo!()
    }
}
