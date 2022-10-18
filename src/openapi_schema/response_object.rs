use schemars::Map;
use serde::Serialize;

use super::{media_type_object::MediaTypeObject, reference_object::Referenceable};

pub type ResponsesObject = Map<String, Referenceable<ResponseObject>>;

#[derive(Debug, Serialize)]
pub struct ResponseObject {
    pub description: String,
    pub contents: Map<String, MediaTypeObject>,
}
