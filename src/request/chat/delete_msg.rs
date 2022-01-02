/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::DELETEMSG;
use crate::RequestAndResponse;
use crate::response::chat::DeleteMsgRes;

/// Delete chat. Official server only deletes message sent before 5 mins max.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMsgReq {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Chat log id
    #[serde(rename = "logId")]
    pub log_id: i64

}

impl RequestAndResponse for DeleteMsgReq {
    const METHOD: &'static str = DELETEMSG;
    type Response = DeleteMsgRes;
}