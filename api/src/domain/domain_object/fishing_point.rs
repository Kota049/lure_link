use serde::Serialize;
use crate::impl_text_rule;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Destination(String);

impl_text_rule!(Destination,30usize);
