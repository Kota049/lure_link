use serde::Serialize;
use crate::impl_integer_rule;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Budget(i32);

impl_integer_rule!(Budget,1000000);