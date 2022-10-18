use schemars::schema::Schema;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MediaTypeObject {
    pub schema: Schema,
}
