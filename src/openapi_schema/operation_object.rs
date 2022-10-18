use serde::Serialize;

use super::response_object::ResponsesObject;

#[derive(Debug, Serialize)]
pub struct OperationObject {
    pub responses: ResponsesObject,
}
