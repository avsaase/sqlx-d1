use sqlx_core::{arguments::Arguments, encode::Encode, types::Type};

use crate::database::D1;

#[derive(Debug, Default)]
pub struct D1Arguments;

impl<'q> Arguments<'q> for D1Arguments {
    type Database = D1;

    fn reserve(&mut self, additional: usize, size: usize) {
        todo!()
    }

    fn add<T>(&mut self, value: T)
    where
        T: Encode<'q, Self::Database> + Type<Self::Database>,
    {
        todo!()
    }
}
