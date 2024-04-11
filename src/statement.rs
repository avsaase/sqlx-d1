use std::marker::PhantomData;

use sqlx_core::{impl_statement_query, statement::Statement, Either};

use crate::{arguments::D1Arguments, column::D1Column, database::D1, type_info::D1TypeInfo};

pub struct D1Statement<'q>(PhantomData<&'q ()>);

impl<'q> Statement<'q> for D1Statement<'q> {
    type Database = D1;

    fn to_owned(&self) -> D1Statement<'static> {
        todo!()
    }

    fn sql(&self) -> &str {
        todo!()
    }

    fn parameters(&self) -> Option<Either<&[D1TypeInfo], usize>> {
        todo!()
    }

    fn columns(&self) -> &[D1Column] {
        todo!()
    }

    impl_statement_query!(D1Arguments);
}
