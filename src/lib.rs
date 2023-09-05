//! Managing consent with Yew
//!
//! # Usage
//!
//! The idea is to ask the user for some kind of consent, which is modeled by
//! [`ConsentState`] plus the additional type `T` (which can also be `()`) if no further
//! specializations are needed.
//!
//! Wrap the main content of your application with the [`component::Consent`] component. It will
//! load the consent state from the browser storage, and then either show the `ask` content, which
//! is responsible to get the consent state. Or it will show the "children", wrapping them with the
//! consent state as defined by the user.
//!
//! All component below the [`component::Consent`] component can use the [`hook::use_consent`] hook,
//! to fetch the [`ConsentState`]. They can also use the [`hook::use_consent_context`] to change
//! the consent state later on.
//!
//! # Examples
//!
//! See the example project in the `example` folder.
use std::fmt::Debug;

pub mod component;
pub mod hook;
pub mod prelude;

/// The state of the consent.
///
/// There are two basic states: [`ConsentState::No`], which means that the user rejects everything.
/// And [`ConsentState::Yes`], which means that the user agrees, but there may be additional
/// customizations, based on the type of `T` being used.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsentState<T> {
    /// The user consents. May have additional payload, use `()` if none.
    Yes(T),
    /// The user does not consent.
    #[default]
    No,
}
