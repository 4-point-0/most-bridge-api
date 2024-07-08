use actix_web::web::ServiceConfig;
use actix_web::{post, web, Error as ActixError};
use base64::{self, engine::general_purpose::STANDARD, Engine};
use constants::{GAS_BUDGET, GAS_OBJECT_ID, MAINNET_URL, MODE, SIGNER, TESTNET_URL};
use fastcrypto::{
    encoding::{Base64, Encoding},
    hash::HashFunction,
};
use models::Reply;
use models::{PaySuiRequest, PaySuiResponse, TxDigestRequest};
use shared_crypto::intent::{Intent, IntentMessage};
use shuttle_actix_web::ShuttleActixWeb;
use sui_types::transaction::TransactionData;
mod constants;
mod models;

#[post("/tx-digest")]
async fn tx_digest(dto: web::Json<TxDigestRequest>) -> Result<Reply, ActixError> {
    match transfer_sui(dto.clone()).await {
        Ok(tx_bytes) => {
            let decoded = Engine::decode(&STANDARD, tx_bytes.clone()).unwrap();
            let tx_data: TransactionData = bcs::from_bytes(&decoded).unwrap();

            let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data);

            let raw_intent_msg: Vec<u8> = match bcs::to_bytes(&intent_msg) {
                Ok(bytes) => bytes,
                Err(err) => panic!("Failed to serialize intent message: {}", err.to_string()),
            };

            let mut hasher = sui_types::crypto::DefaultHash::default();
            hasher.update(raw_intent_msg);
            let dig = hasher.finalize().digest;
            let digest = Base64::encode(dig);

            return Ok(Reply {
                digest,
                tx_bytes: tx_bytes.clone(),
            });
        }
        Err(err) => panic!("Failed to get tx digest: {}", err.to_string()),
    }
}

async fn transfer_sui(dto: TxDigestRequest) -> Result<std::string::String, reqwest::Error> {
    let model: PaySuiRequest = PaySuiRequest {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "unsafe_transferSui".to_string(),
        params: vec![
            SIGNER.to_string(),        //signer
            GAS_OBJECT_ID.to_string(), //sui_object_id
            GAS_BUDGET.to_string(),    //gas budget
            dto.recipient,             //recipient
            dto.amount,                //amount
        ],
    };

    let response = reqwest::Client::new()
        .post(match MODE {
            "dev" => TESTNET_URL.to_string(),
            _ => MAINNET_URL.to_string(),
        })
        .json(&model)
        .send()
        .await?;

    let resp = response.json::<PaySuiResponse>().await?;

    return Ok(resp.result.tx_bytes);
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(tx_digest);
    };

    Ok(config.into())
}
