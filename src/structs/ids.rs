use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChannelId(i64);

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserId(i64);
