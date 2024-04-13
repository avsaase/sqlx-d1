use sqlx_core::{
    arguments::Arguments,
    encode::{Encode, IsNull},
    types::Type,
};

use crate::database::D1;

pub enum D1ArgumentValue<'q> {
    Null,
    Real(f64),
    Integer(i32),
    Text(&'q str),
    Boolean(bool),
    Blob(&'q [u8]),
}

#[derive(Default)]
pub struct D1Arguments<'q> {
    pub(crate) values: Vec<D1ArgumentValue<'q>>,
}

impl<'q> D1Arguments<'q> {
    fn add<T>(&mut self, value: T)
    where
        T: Encode<'q, D1>,
    {
        // let ty = T::type_info();

        if let IsNull::Yes = value.encode(&mut self.values) {
            self.values.push(D1ArgumentValue::Null);
        }
    }
}

impl<'q> Arguments<'q> for D1Arguments<'q> {
    type Database = D1;

    fn reserve(&mut self, additional: usize, _size: usize) {
        self.values.reserve(additional);
    }

    fn add<T>(&mut self, value: T)
    where
        T: Encode<'q, Self::Database> + Type<Self::Database> + Send + 'q,
    {
        self.add(value)
    }
}
