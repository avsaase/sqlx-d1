use sqlx_core::column::Column;

use crate::{database::D1, type_info::D1TypeInfo};

#[derive(Debug)]
pub struct D1Column;

impl Column for D1Column {
    type Database = D1;

    fn ordinal(&self) -> usize {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn type_info(&self) -> &D1TypeInfo {
        todo!()
    }
}
