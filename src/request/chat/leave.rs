/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::LEAVE;
use crate::RequestAndResponse;
use crate::response::chat::LeaveRes;

/// Leave chatroom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveReq {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Block chatroom. Cannot rejoin chatroom if true.
    pub block: bool

}

impl RequestAndResponse for LeaveReq {
    const METHOD: &'static str = LEAVE;
    type Response = LeaveRes;
}