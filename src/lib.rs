use std::fmt::Debug;

pub mod component;
pub mod hook;
pub mod prelude;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsentState<T> {
    Yes(T),
    #[default]
    No,
}
