use actix_web::{error, post, web, Error, HttpRequest};
use futures::StreamExt;
use hmac::{Hmac, Mac};
use serde::{Deserialize};
use sha2::Sha256;

use crate::env_helpers::cast_required_env_var;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FullTransaction {
    hash: String,
    #[serde(rename = "blockHash")]
    block_hash: String,
    #[serde(rename = "blockNumber")]
    block_number: String,
    from: String,
    gas: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,
    input: String,
    nonce: String,
    r: String,
    s: String,
    to: String,
    #[serde(rename = "transactionIndex")]
    transaction_index: String,
    v: String,
    value: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Webhook {
    app: String,
    network: String,
    #[serde(rename = "webhookType")]
    webhook_type: String,
    hash: String,
    timestamp: String,
    #[serde(rename = "fullTransaction")]
    full_transaction: FullTransaction,
}

fn is_valid_signature_for_string_body(
    body: &[u8],       // must be raw string body, not json transformed version of the body
    signature: &str,   // your "X-Alchemy-Signature" from header
    signing_key: &str, // taken from dashboard for specific webhook
) -> Result<bool, Box<dyn std::error::Error>> {
    let signing_key_bytes: Vec<u8> = signing_key.bytes().collect();
    let mut mac = Hmac::<Sha256>::new_from_slice(&signing_key_bytes)?;
    mac.update(&body);
    let hex_decode_signature = hex::decode(signature)?;
    let verification = mac.verify_slice(&hex_decode_signature).is_ok();
    Ok(verification)
}

const MAX_SIZE: usize = 262_144;

#[post("/webhook")]
pub async fn webhook(mut payload: web::Payload, req: HttpRequest) -> Result<String, Error> {
    let signature_header = req
        .headers()
        .get("X-Alchemy-Signature")
        .ok_or(error::ErrorBadRequest(
            "Missing signature header",
        ))?
        .to_str()
        .or(Err(error::ErrorBadRequest(
            "Invalid signature header",
        )))?;

    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // Check if the signature is valid
    let signing_key = cast_required_env_var::<String>("SIGNING_KEY");
    let is_valid =
        is_valid_signature_for_string_body(&body, signature_header, signing_key.as_str())?;

    if !is_valid {
        return Err(error::ErrorBadRequest("Invalid signature"));
    }

    Ok("ok".to_string())
}
