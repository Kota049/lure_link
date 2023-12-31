use serde::Serialize;
use crate::impl_integer_rule;

const BUDGET_MAX: i32 = 1000000;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Budget(i32);

impl_integer_rule!(Budget,BUDGET_MAX);