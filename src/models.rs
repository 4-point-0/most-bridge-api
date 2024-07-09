use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcRequest {
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteTxBlockRequest {
    pub signature: String,
    pub tx_bytes: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteTxBlockResponse {
    pub jsonrpc: String,
    pub result: ExecuteTxBlockResult,
    pub id: i64,
}

impl Responder for ExecuteTxBlockResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteTxBlockResult {
    pub digest: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct QueryEventsRpcRequest {
    pub jsonrpc: String,
    pub id: i64,
    pub method: String,
    pub params: Vec<QueryParamElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParamClassOne {
    #[serde(rename = "MoveModule")]
    pub move_module: Option<MoveModule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParamClassTwo {
    #[serde(rename = "txDigest")]
    pub tx_digest: Option<String>,

    #[serde(rename = "eventSeq")]
    pub event_seq: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveModule {
    #[serde(rename = "package")]
    pub package: String,

    #[serde(rename = "module")]
    pub module: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum QueryParamElement {
    Bool(bool),

    Integer(i64),

    QueryParamClassOne(QueryParamClassOne),
    Null,
    QueryParamClassTwo(QueryParamClassTwo),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryEventsRequest {
    pub tx_digest: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRpcResponse {
    pub jsonrpc: String,
    pub result: QueryRpcResponseResult,
    pub id: i64,
}

impl Responder for QueryRpcResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRpcResponseResult {
    pub data: Vec<QueryRpcResponseResultData>,
    pub next_cursor: NextCursor,
    pub has_next_page: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRpcResponseResultData {
    pub id: Id,
    pub package_id: String,
    pub transaction_module: String,
    pub sender: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub parsed_json: ParsedJson,
    pub bcs: String,
    pub timestamp_ms: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    pub tx_digest: String,
    pub event_seq: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedJson {
    pub from: String,
    #[serde(rename = "minter_address")]
    pub minter_address: String,
    #[serde(rename = "principal_address")]
    pub principal_address: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextCursor {
    pub tx_digest: String,
    pub event_seq: String,
}
