use crate::impl_text_rule;
use serde::Serialize;

const MAX_APPLICATION_TOKEN: usize = 20usize;

#[derive(Debug, Clone, Serialize, PartialOrd, PartialEq)]
pub struct ApplicationToken(String);

impl_text_rule!(ApplicationToken, MAX_APPLICATION_TOKEN);
