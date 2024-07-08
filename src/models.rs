use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaySuiRequest {
    pub jsonrpc: String,
    pub id: i64,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaySuiResponse {
    pub jsonrpc: String,
    pub result: PaySuiResponseResult,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaySuiResponseResult {
    pub tx_bytes: String,
    pub gas: Vec<Gas>,
    pub input_objects: Vec<InputObject>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gas {
    pub object_id: String,
    pub version: i64,
    pub digest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputObject {
    #[serde(rename = "ImmOrOwnedMoveObject")]
    pub imm_or_owned_move_object: ImmOrOwnedMoveObject,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmOrOwnedMoveObject {
    pub object_id: String,
    pub version: i64,
    pub digest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceGasPriceRequest {
    pub jsonrpc: String,
    pub id: i64,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceGasPriceResponse {
    pub jsonrpc: String,
    pub result: String,
    pub id: i64,
}
#[derive(Deserialize, Clone)]
pub struct TxDigestRequest {
    pub recipient: String,
    pub amount: String,
}

#[derive(Serialize, Clone)]
pub struct Reply {
    pub digest: String,
    pub tx_bytes: String,
}

impl Responder for Reply {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
