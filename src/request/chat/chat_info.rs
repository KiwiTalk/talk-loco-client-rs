/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Deserialize, Serialize};
use crate::method::CHATINFO;
use crate::RequestAndResponse;
use crate::response::chat::ChatInfoRes;
use crate::structs::chat::ChatId;

/// Request Chatroom info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInfoReq {
    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: ChatId,
}

impl RequestAndResponse for ChatInfoReq {
    const METHOD: &'static str = CHATINFO;
    type Response = ChatInfoRes;
}