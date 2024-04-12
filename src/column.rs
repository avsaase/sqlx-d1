use sqlx_core::{column::Column, ext::ustr::UStr};

use crate::{database::D1, type_info::D1TypeInfo};

#[derive(Debug)]
pub struct D1Column {
    name: UStr,
    orginal: usize,
    type_info: D1TypeInfo,
}

impl Column for D1Column {
    type Database = D1;

    fn ordinal(&self) -> usize {
        self.orginal
    }

    fn name(&self) -> &str {
        &*self.name
    }

    fn type_info(&self) -> &D1TypeInfo {
        &self.type_info
    }
}
