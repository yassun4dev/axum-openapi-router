use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};

use super::json_schema_ref::JsonSchemaRef;

pub trait FromRequest {
    fn from_request(gen: &mut SchemaGenerator) -> Schema;
}

impl<T> FromRequest for axum::Json<T>
where
    T: JsonSchema,
{
    fn from_request(gen: &mut SchemaGenerator) -> Schema {
        JsonSchemaRef::<T>::json_schema(gen)
    }
}
