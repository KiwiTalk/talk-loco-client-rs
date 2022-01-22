/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//todo User와 OpenUser겹치는부분 통합

use serde::{Deserialize, Serialize};
use crate::structs::ids::UserId;

/// Minimal user info for chatroom display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayUserInfo {
    /// User id
    #[serde(rename = "userId")]
    pub user_id: UserId,

    /// User nickname
    #[serde(rename = "nickName")]
    pub nickname: String,

    /// Profile image URL. None if profile image is default.
    #[serde(rename = "pi")]
    pub profile_image_url: Option<String>,

    /// Country Iso, does not present on openchat.
    #[serde(rename = "countryIso", skip_serializing_if = "Option::is_none")]
    pub country_iso: Option<String>,
}

/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: i64,

    #[serde(rename = "nickName")]
    pub nickname: String,

    #[serde(rename = "countryIso")]
    pub country_iso: String,

    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: Option<String>,

    #[serde(rename = "fullProfileImageUrl")]
    pub full_profile_image_url: Option<String>,

    #[serde(rename = "OriginalProfileImageUrl")]
    pub original_profile_image_url: Option<String>,

    /// See UserType for types.
    #[serde(rename = "type")]
    pub user_type: i32,

    #[serde(rename = "accountId")]
    pub account_id: i64,

    #[serde(rename = "linkedServices")]
    pub linked_services: String,

    #[serde(rename = "statusMessage")]
    pub status_message: String,

    pub suspended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserVariant {
    Normal(User),
    Open(OpenUser),
}

/// User types. Don't confuse with OpenMemberType.
#[repr(i32)]
pub enum UserType {
    Unknown = -999999,
    NotFriend = -100,
    Deactivated = 9,
    Friend = 100,
    Openchat = 1000,
}

/// Openchat user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenUser {

    #[serde(rename = "userId")]
    pub user_id: i64,

    #[serde(rename = "nickName")]
    pub nickname: String,

    #[serde(rename = "pi")]
    pub profile_image_url: Option<String>,

    #[serde(rename = "fpi")]
    pub full_profile_image_url: Option<String>,

    #[serde(rename = "opi")]
    pub original_profile_image_url: Option<String>,

    /// See `struct/user` UserType for types.
    #[serde(rename = "type")]
    pub user_type: i32,

    /// See OpenMemberType for types.
    #[serde(rename = "mt")]
    pub open_member_type: i8,

    #[serde(rename = "opt")]
    pub open_token: i32,

    /// Profile link id. Only presents if user using openlink profile.
    #[serde(rename = "pli", skip_serializing_if = "Option::is_none")]
    pub profile_link_id: Option<i64>,

}
