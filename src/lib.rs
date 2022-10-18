#![allow(dead_code)]

mod openapi_generator;
mod openapi_schema;
mod router;

pub use openapi_generator::OpenApiGenerator;
pub use openapi_schema::OpenApiSchema;

#[cfg(test)]
mod test {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::OpenApiGenerator;
    type TestResult = Result<(), ()>;

    #[test]
    fn openapi_generator() -> TestResult {
        #[derive(JsonSchema, Deserialize)]
        struct RequestBody {
            name: String,
        }

        #[derive(JsonSchema, Serialize)]
        struct ResponseBody {
            name: String,
        }

        async fn handler(axum::Json(_): axum::Json<RequestBody>) -> axum::Json<ResponseBody> {
            unimplemented!()
        }

        let schema = OpenApiGenerator::default()
            .route::<axum::Json<RequestBody>, axum::Json<ResponseBody>>(
                "/users",
                crate::router::get(handler),
            )
            .generate();
        assert_eq!(
            serde_json::to_value(schema).unwrap(),
            json!({
                "version": "V3_0_3",
                "info": {
                    "title": "title",
                    "version": "1.0"
                },
                "paths": {
                    "/users": {
                        "get": {
                            "responses": {
                                "200": {
                                    "description": "",
                                    "contents": {
                                        "application/json": {
                                            "schema": {
                                                "$ref": "#/components/schemas/ResponseBody"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "components": {
                    "schemas": {
                        "ResponseBody": {
                            "type": "object",
                            "required": [
                                "name"
                            ],
                            "properties": {
                                "name": {
                                    "type": "string"
                                }
                            }
                        }
                    }
                }
            })
        );
        Ok(())
    }
}
