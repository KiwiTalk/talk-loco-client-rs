/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::SETMETA;
use crate::RequestAndResponse;
use crate::response::chat::SetMetaRes;

/// Set Chatroom meta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMetaReq {

    /// Chatroom id
    #[serde(rename = "chatId")]
    pub chat_id: i64,

    /// Meta type. See `structs/chatroom.rs` ChatroomMetaType for predefined types.
    #[serde(rename = "type")]
    pub meta_type: i8,

    /// Json or String content. Different depending on type.
    pub content: String

}

impl RequestAndResponse for SetMetaReq {
    const METHOD: &'static str = SETMETA;
    type Response = SetMetaRes;
}