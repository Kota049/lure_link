use serde::Serialize;
use crate::impl_text_rule;
const MAX_MUNICIPALITY_NAME:usize = 20usize;
#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct Municipality(String);

impl_text_rule!(Municipality,MAX_MUNICIPALITY_NAME);