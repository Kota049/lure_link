use serde::Serialize;
use crate::impl_text_rule;
const MAX_POINT_NAME:usize = 30usize;
#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct PointName(String);

impl_text_rule!(PointName,MAX_POINT_NAME);
