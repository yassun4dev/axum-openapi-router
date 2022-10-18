mod from_request;
mod into_response;
mod json_schema_ref;

use crate::{
    openapi_schema::{
        self, Components, MediaTypeObject, OperationObject, PathItemObject, Referenceable,
        ResponseObject,
    },
    router::PathSchema,
    OpenApiSchema,
};
use axum::http::Method;
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    Map,
};

pub use self::from_request::FromRequest;
pub use self::into_response::IntoResponse;

pub struct OpenApiGenerator {
    title: String,
    version: String,
    paths: Map<String, PathItemObject>,
    generator: SchemaGenerator,
}

impl OpenApiGenerator {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn route<Request, Response>(
        mut self,
        path: &str,
        service: PathSchema<Request, Response>,
    ) -> Self
    where
        Request: FromRequest,
        Response: IntoResponse,
    {
        match *service.method() {
            Method::GET => {
                let path_item = self.paths.entry(path.to_owned()).or_default();
                path_item.get = Some(OperationObject {
                    responses: response200(Referenceable::Object(ResponseObject {
                        description: "".to_owned(),
                        contents: application_json(MediaTypeObject {
                            schema: Response::into_response(&mut self.generator),
                        }),
                    })),
                });
            }
            _ => (),
        };

        self
    }

    pub fn generate(self) -> OpenApiSchema {
        OpenApiSchema {
            version: openapi_schema::Version::V3_0_3,
            paths: self.paths,
            components: Components {
                schemas: self.generator.definitions().clone(),
            },
            info: openapi_schema::Info {
                title: self.title,
                version: self.version,
            },
        }
    }
}

impl Default for OpenApiGenerator {
    fn default() -> Self {
        Self {
            title: "title".to_owned(),
            version: "1.0".to_owned(),
            paths: Default::default(),
            generator: SchemaGenerator::from(SchemaSettings::openapi3()),
        }
    }
}

fn response200(media: Referenceable<ResponseObject>) -> Map<String, Referenceable<ResponseObject>> {
    let mut contents = Map::new();

    contents.insert("200".to_owned(), media);

    contents
}

fn application_json(media: MediaTypeObject) -> Map<String, MediaTypeObject> {
    let mut contents = Map::new();

    contents.insert("application/json".to_owned(), media);

    contents
}
