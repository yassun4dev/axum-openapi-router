use schemars::JsonSchema;

#[derive(JsonSchema)]
pub struct JsonSchemaRef<T: JsonSchema>(T);
