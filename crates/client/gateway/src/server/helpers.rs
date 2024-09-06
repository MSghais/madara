use std::collections::HashMap;

use hyper::{header, Body, Request, Response, StatusCode};
use mp_block::{BlockId, BlockTag};
use serde::Serialize;
use starknet_types_core::felt::Felt;

use crate::error::{StarknetError, StarknetErrorCode};

pub(crate) fn service_unavailable_response(service_name: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .body(Body::from(format!("{} Service disabled", service_name)))
        .expect("Failed to build SERVICE_UNAVAILABLE response with a valid status and body")
}

pub(crate) fn not_found_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not Found"))
        .expect("Failed to build NOT_FOUND response with a valid status and body")
}

pub(crate) fn internal_error_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .expect("Failed to build INTERNAL_SERVER_ERROR response with a valid status and body")
}

pub(crate) fn not_implemented_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(Body::from("Not Implemented"))
        .expect("Failed to build NOT_IMPLEMENTED response with a valid status and body")
}

/// Creates a JSON response with the given status code and a body that can be serialized to JSON.
///
/// If the serialization fails, this function returns a 500 Internal Server Error response.
pub(crate) fn create_json_response<T>(status: StatusCode, body: &T) -> Response<Body>
where
    T: Serialize,
{
    // Serialize the body to JSON
    let body = match serde_json::to_string(body) {
        Ok(body) => body,
        Err(e) => {
            log::error!("Failed to serialize response body: {}", e);
            return internal_error_response();
        }
    };

    // Build the response with the specified status code and serialized body
    match Response::builder().status(status).header(header::CONTENT_TYPE, "application/json").body(Body::from(body)) {
        Ok(response) => response,
        Err(e) => {
            log::error!("Failed to build response: {}", e);
            internal_error_response()
        }
    }
}

pub(crate) fn create_response_with_json_body(status: StatusCode, body: &str) -> Response<Body> {
    // Build the response with the specified status code and serialized body
    match Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
    {
        Ok(response) => response,
        Err(e) => {
            log::error!("Failed to build response: {}", e);
            internal_error_response()
        }
    }
}

pub(crate) fn get_params_from_request(req: &Request<Body>) -> HashMap<String, String> {
    let query = req.uri().query().unwrap_or("");
    let params = query.split('&');
    let mut query_params = HashMap::new();
    for param in params {
        let parts: Vec<&str> = param.split('=').collect();
        if parts.len() == 2 {
            query_params.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    query_params
}

// blockNumber or blockHash
pub(crate) fn block_id_from_params(params: &HashMap<String, String>) -> Result<BlockId, StarknetError> {
    if let Some(block_number) = params.get("blockNumber") {
        match block_number.as_str() {
            "latest" => Ok(BlockId::Tag(BlockTag::Latest)),
            "pending" => Ok(BlockId::Tag(BlockTag::Pending)),
            _ => {
                let block_number = block_number.parse().map_err(|e: std::num::ParseIntError| {
                    StarknetError::new(StarknetErrorCode::MalformedRequest, e.to_string())
                })?;
                Ok(BlockId::Number(block_number))
            }
        }
    } else if let Some(block_hash) = params.get("blockHash") {
        let block_hash = Felt::from_hex(block_hash)
            .map_err(|e| StarknetError::new(StarknetErrorCode::MalformedRequest, e.to_string()))?;
        Ok(BlockId::Hash(block_hash))
    } else {
        Err(StarknetError::new(StarknetErrorCode::MalformedRequest, "block_number or block_hash not found".to_string()))
    }
}

// append_pair("includeBlock", "true");

pub(crate) fn include_block_params(params: &HashMap<String, String>) -> bool {
    params.get("includeBlock").map_or(false, |v| v == "true")
}

impl From<StarknetError> for hyper::Response<hyper::Body> {
    fn from(error: StarknetError) -> Self {
        create_json_response(hyper::StatusCode::BAD_REQUEST, &error)
    }
}