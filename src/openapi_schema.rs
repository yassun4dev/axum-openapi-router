mod media_type_object;
mod operation_object;
mod path_object;
mod reference_object;
mod response_object;

use schemars::schema::Schema;

pub use media_type_object::MediaTypeObject;
pub use operation_object::OperationObject;
pub use path_object::{PathItemObject, PathsObject};
pub use reference_object::{ReferenceObject, Referenceable};
pub use response_object::ResponseObject;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Version {
    V3_0_3,
}

#[derive(Debug, Serialize)]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct Components {
    pub schemas: schemars::Map<String, Schema>,
}

#[derive(Debug, Serialize)]
pub struct OpenApiSchema {
    pub version: Version,
    pub info: Info,
    pub paths: PathsObject,
    pub components: Components,
}
