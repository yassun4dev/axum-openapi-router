use schemars::Map;
use serde::Serialize;

use super::operation_object::OperationObject;

pub type PathsObject = Map<String, PathItemObject>;

#[derive(Debug, Default, Serialize)]
pub struct PathItemObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OperationObject>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OperationObject>,
}
