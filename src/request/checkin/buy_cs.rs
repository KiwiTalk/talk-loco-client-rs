/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{ structs::client::ClientInfo};

/// Request call server host data.
/// Checkin response already contains call server info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyCSReq {

    #[serde(flatten)]
    pub client: ClientInfo,

    #[serde(rename = "countryISO")]
    pub country_iso: String,

}
