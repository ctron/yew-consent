use crate::prelude::{ConsentContext, ConsentState};
use yew::prelude::*;

/// Get the current consent state. If no consent state is found, it defaults to
/// [`ConsentState::No`].
#[hook]
pub fn use_consent<T>() -> ConsentState<T>
where
    for<'de> T: Clone + PartialEq + 'static,
{
    use_context().unwrap_or_default()
}

/// Get the consent context. It can be used to manipulate the consent state. The hook will return
/// [`Option::None`] if the call is not made from a component wrapped by the
/// [`crate::component::Consent`] component.
#[hook]
pub fn use_consent_context<T>() -> Option<ConsentContext<T>>
where
    T: Clone + PartialEq + 'static,
{
    use_context::<ConsentContext<T>>()
}
