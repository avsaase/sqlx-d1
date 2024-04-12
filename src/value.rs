use std::borrow::Cow;

use sqlx_core::value::{Value, ValueRef};

use crate::{database::D1, type_info::D1TypeInfo};

pub struct D1ValueRef<'r>(D1ValueData<'r>);

pub struct D1ValueData<'r>(&'r D1Value);

impl<'r> ValueRef<'r> for D1ValueRef<'r> {
    type Database = D1;

    fn to_owned(&self) -> D1Value {
        todo!()
    }

    fn type_info(&self) -> Cow<'_, D1TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}

impl<'r> D1ValueRef<'r> {
    pub(crate) fn value(value: &'r D1Value) -> Self {
        Self(D1ValueData(&value))
    }
}

pub struct D1Value;

impl Value for D1Value {
    type Database = D1;

    fn as_ref(&self) -> D1ValueRef<'_> {
        todo!()
    }

    fn type_info(&self) -> Cow<'_, D1TypeInfo> {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }
}
