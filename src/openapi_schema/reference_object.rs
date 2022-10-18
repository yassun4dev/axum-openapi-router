use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ReferenceObject(String);

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Referenceable<T> {
    Object(T),
    Ref(ReferenceObject),
}
