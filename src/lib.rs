/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//! Official client/server compatible Loco client and commands implementation.
//! See [request], [response] module for command datas.
//! See [structs] module for types used in command datas.
//! See [client] module for client implementation.

pub mod request;
pub mod response;

pub mod structs;

pub mod command;
mod stream;

pub mod client;
