use std::{future::Future, marker::PhantomData};

use crate::openapi_generator::{FromRequest, IntoResponse};
use axum::http::Method;

pub struct PathSchema<Request, Response> {
    method: Method,
    phantom: PhantomData<(Request, Response)>,
}

impl<Request, Response> PathSchema<Request, Response> {
    pub fn method(&self) -> &Method {
        &self.method
    }
}

pub fn get<Request, Response, ResponseFuture>(
    _handler: fn(Request) -> ResponseFuture,
) -> PathSchema<Request, Response>
where
    Request: FromRequest,
    Response: IntoResponse,
    ResponseFuture: Future<Output = Response>,
{
    PathSchema {
        method: Method::GET,
        phantom: PhantomData,
    }
}
