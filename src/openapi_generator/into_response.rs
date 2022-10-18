use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};

use super::json_schema_ref::JsonSchemaRef;

pub trait IntoResponse {
    fn into_response(gen: &mut SchemaGenerator) -> Schema;
}

impl<T> IntoResponse for axum::Json<T>
where
    T: JsonSchema,
{
    fn into_response(gen: &mut SchemaGenerator) -> Schema {
        JsonSchemaRef::<T>::json_schema(gen)
    }
}
