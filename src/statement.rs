use std::{borrow::Cow, marker::PhantomData, sync::Arc};

use sqlx_core::{
    column::ColumnIndex, ext::ustr::UStr, impl_statement_query, statement::Statement, Either,
    Error, HashMap,
};

use crate::{arguments::D1Arguments, column::D1Column, database::D1, type_info::D1TypeInfo};

pub struct D1Statement<'q> {
    sql: Cow<'q, str>,
    parameters: usize,
    columns: Arc<Vec<D1Column>>,
    column_names: Arc<HashMap<UStr, usize>>,
}

impl<'q> Statement<'q> for D1Statement<'q> {
    type Database = D1;

    fn to_owned(&self) -> D1Statement<'static> {
        D1Statement {
            sql: Cow::Owned(self.sql.to_string()),
            parameters: self.parameters,
            columns: Arc::clone(&self.columns),
            column_names: Arc::clone(&self.column_names),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[D1TypeInfo], usize>> {
        Some(Either::Right(self.parameters))
    }

    fn columns(&self) -> &[D1Column] {
        &self.columns
    }

    impl_statement_query!(D1Arguments);
}

impl ColumnIndex<D1Statement<'_>> for &str {
    fn index(&self, statement: &D1Statement) -> Result<usize, Error> {
        statement
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
