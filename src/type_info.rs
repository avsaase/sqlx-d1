use std::fmt::{self, Display, Formatter};

use sqlx_core::type_info::TypeInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct D1TypeInfo;

impl TypeInfo for D1TypeInfo {
    fn is_null(&self) -> bool {
        todo!()
    }

    fn name(&self) -> &str {
        todo!()
    }
}

impl Display for D1TypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
