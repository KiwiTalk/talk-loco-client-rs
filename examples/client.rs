/*
 * Created on Wed Dec 08 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::error::Error;

use loco_protocol::secure::{
    crypto::CryptoStore, session::SecureClientSession, stream::SecureStream,
};
use rsa::{pkcs8::FromPublicKey, RsaPublicKey};
use talk_loco_client::{
    command::{manager::BsonCommandManager, session::BsonCommandSession, BsonCommand},
    request,
    response::{self, ResponseData},
    stream::ChunkedWriteStream,
    structs::client::ClientInfo,
};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

pub static KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIDANBgkqhkiG9w0BAQEFAAOCAQ0AMIIBCAKCAQEApElgRBx+g7sniYFW7LE8ivrwXShKTRFV8lXNItMXbN5QSC8vJ/cTSOTS619Xv5Zx7xXJIk4EKxtWesEGbgZpEUP2xQ+IeH9oz0JxayEMvvD1nVNAWgpWE4pociEoArsK7qY3YwXb1CiDHo9hojLv7djbo3cwXvlyMh4TUrX2RjCZPlVJxk/LVjzcl9ohJLkl3eoSrf0AE4kQ9mk3+raEhq5Dv+IDxKYX+fIytUWKmrQJusjtre9oVUX5sBOYZ0dzez/XapusEhUWImmB6mciVXfRXQ8IK4IH6vfNyxMSOTfLEhRYN2SMLzplAYFiMV536tLS3VmG5GJRdkpDubqPeQIBAw==
-----END PUBLIC KEY-----";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let loco_session = SecureClientSession::new(
        RsaPublicKey::from_public_key_der(&pem::parse(KEY)?.contents).unwrap(),
    );

    let mut stream = SecureStream::new(
        CryptoStore::new(),
        ChunkedWriteStream::new(
            TcpStream::connect("ticket-loco.kakao.com:443")
                .await
                .unwrap()
                .compat(),
            2048,
        ),
    );

    loco_session.handshake_async(&mut stream).await?;

    let mut checkin_conn = BsonCommandSession::new(BsonCommandManager::new(stream));

    let checkin_req = checkin_conn
        .request_async(&BsonCommand::new_const(
            "CHECKIN",
            0,
            request::checkin::Checkin {
                user_id: 1,
                client: ClientInfo {
                    os: "win32".into(),
                    net_type: 0,
                    app_version: "3.2.8".into(),
                    mccmnc: "999".into(),
                },
                language: "ko".into(),
                country_iso: "KR".into(),
                use_usb: true,
            },
        ))
        .await?;

    let checkin_res = checkin_conn.response_async(checkin_req).await?;
    let checkin_res = checkin_res.try_deserialize::<ResponseData<response::checkin::Checkin>>().unwrap();

    println!("CHECKIN response: {:?}", checkin_res);

    Ok(())
}
