use actix_web::web::ServiceConfig;
use actix_web::{get, post, web, Error as ActixError};
use base64::{self, engine::general_purpose::STANDARD, Engine};
use constants::{
    GAS_BUDGET, GAS_OBJECT_ID, MAINNET_URL, MODE, SIGNER, SUI_MODULE_ID, SUI_PACKAGE_ID,
    TESTNET_URL,
};
use fastcrypto::{
    encoding::{Base64, Encoding},
    hash::HashFunction,
};

use models::{
    ExecuteTxBlockRequest, ExecuteTxBlockResponse, MoveModule, QueryEventsRequest,
    QueryEventsRpcRequest, QueryParamClassOne, QueryParamClassTwo, QueryParamElement,
    QueryRpcResponse, Reply,
};
use models::{PaySuiResponse, RpcRequest, TxDigestRequest};
use shared_crypto::intent::{Intent, IntentMessage};
use shuttle_actix_web::ShuttleActixWeb;
use sui_types::transaction::TransactionData;
mod constants;
mod models;

#[post("/tx-digest")]
async fn get_tx_digest(dto: web::Json<TxDigestRequest>) -> Result<Reply, ActixError> {
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

#[post("/execute-tx-block")]
async fn execute_tx_block(
    dto: web::Json<ExecuteTxBlockRequest>,
) -> Result<ExecuteTxBlockResponse, ActixError> {
    let ExecuteTxBlockRequest {
        signature,
        tx_bytes,
    } = dto.clone();
    let model: RpcRequest = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "sui_executeTransactionBlock".to_string(),
        params: vec![signature, tx_bytes],
    };

    match reqwest::Client::new()
        .post(match MODE {
            "dev" => TESTNET_URL.to_string(),
            _ => MAINNET_URL.to_string(),
        })
        .json(&model)
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<ExecuteTxBlockResponse>().await {
                Ok(resp) => return Ok(resp),
                Err(err) => panic!("Failed to execute tx block: {}", err.to_string()),
            };
        }
        Err(err) => panic!("Failed to execute tx block: {}", err.to_string()),
    };
}

#[post("/query-events")]
async fn sui_events(
    dto: web::Json<QueryEventsRequest>,
) -> Result<QueryRpcResponse, Box<dyn std::error::Error>> {
    let QueryEventsRequest { tx_digest } = dto.clone();

    let mut params: Vec<QueryParamElement> = vec![];

    let param_1 = QueryParamElement::QueryParamClassOne(QueryParamClassOne {
        move_module: Some(MoveModule {
            package: SUI_PACKAGE_ID.to_string(),
            module: SUI_MODULE_ID.to_string(),
        }),
    });
    let param_2 = QueryParamElement::Null;
    let param_3 = QueryParamElement::Integer(18000);
    let param_4 = QueryParamElement::Bool(false);

    params.push(param_1);
    params.push(param_2);
    params.push(param_3);
    params.push(param_4);

    if tx_digest.len() > 0 {
        let param = QueryParamElement::QueryParamClassTwo(QueryParamClassTwo {
            tx_digest: Some(tx_digest),
            event_seq: Some("0".to_string()),
        });
        params[1] = param;
    }

    let model: QueryEventsRpcRequest = QueryEventsRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "suix_queryEvents".to_string(),
        params: params,
    };

    let response = reqwest::Client::new()
        .post(match MODE {
            "dev" => TESTNET_URL.to_string(),
            _ => MAINNET_URL.to_string(),
        })
        .json(&model)
        .send()
        .await?;

    let str_body = String::from_utf8(response.text().await?.into_bytes())
        .expect("Transformed response is not UTF-8 encoded.");
    let res: QueryRpcResponse = serde_json::from_str(&str_body)?;

    return Ok(res);
}

#[get("/ping")]
async fn ping() -> String {
    return "Pong".to_string();
}

async fn transfer_sui(dto: TxDigestRequest) -> Result<std::string::String, reqwest::Error> {
    let model: RpcRequest = RpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "unsafe_transferSui".to_string(),
        params: vec![
            SIGNER.to_string(),
            GAS_OBJECT_ID.to_string(),
            GAS_BUDGET.to_string(),
            dto.recipient,
            dto.amount,
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
        cfg.service(get_tx_digest)
            .service(execute_tx_block)
            .service(sui_events)
            .service(ping);
    };

    Ok(config.into())
}
