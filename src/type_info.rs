use std::fmt::{self, Display, Formatter};

use sqlx_core::type_info::TypeInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct D1TypeInfo(DataType);

#[derive(Debug, Clone, PartialEq)]
enum DataType {
    Null,
    Real,
    Integer,
    Text,
    Boolean,
    Blob,
}

impl TypeInfo for D1TypeInfo {
    fn is_null(&self) -> bool {
        matches!(self.0, DataType::Null)
    }

    fn name(&self) -> &str {
        match self.0 {
            DataType::Null => "NULL",
            DataType::Real => "REAL",
            DataType::Integer => "INTEGER",
            DataType::Text => "TEXT",
            DataType::Boolean => "BOOLEAN",
            DataType::Blob => "BLOB",
        }
    }
}

impl Display for D1TypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.name())
    }
}
