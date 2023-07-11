use api_request_utils_rs::serde_json::{
    Value,
    to_value
};

use crate::{
    ClassificationName,
    SortingOrder,
    Unit,
    Source,
    Decision
};

impl From<ClassificationName> for Value {
    fn from(name : ClassificationName) -> Self {
        to_value(name).unwrap()
    }
}

impl From<SortingOrder> for Value {
    fn from(name : SortingOrder) -> Self {
        to_value(name).unwrap()
    }
}

impl From<Unit> for Value {
    fn from(name : Unit) -> Self {
        to_value(name).unwrap()
    }
}

impl From<Source> for Value {
    fn from(name : Source) -> Self {
        to_value(name).unwrap()
    }
}

impl From<Decision> for Value {
    fn from(name : Decision) -> Self {
        to_value(name).unwrap()
    }
}