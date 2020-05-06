use std::fmt::{Error, Formatter};

use wasm_bindgen::prelude::*;

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}

impl std::error::Error for FetchError {}

impl From<serde_json::Error> for FetchError {
    fn from(value: serde_json::Error) -> Self {
        FetchError {
            err: value.to_string().into(),
        }
    }
}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

/// The possible states a fetch request can be in.
#[derive(Debug)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

impl<T> Default for FetchState<T> {
    fn default() -> Self {
        Self::NotFetching
    }
}
