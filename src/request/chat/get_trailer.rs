/*
 * Created on Mon Dec 13 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::method::GETTRAILER;
use crate::RequestAndResponse;
use crate::response::chat::GetTrailerRes;

/// Request media download server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrailerReq {
    /// Media key
    #[serde(rename = "k")]
    pub key: String,
    
    /// Chat type
    #[serde(rename = "t")]
    pub chat_type: i32
}

impl RequestAndResponse for GetTrailerReq {
    const METHOD: &'static str = GETTRAILER;
    type Response = GetTrailerRes;
}
