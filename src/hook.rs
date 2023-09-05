use crate::prelude::{ConsentContext, ConsentState};
use yew::prelude::*;

#[hook]
pub fn use_consent<T>() -> ConsentState<T>
where
    for<'de> T: Clone + PartialEq + 'static,
{
    use_context().unwrap_or_default()
}

#[hook]
pub fn use_consent_context<T>() -> Option<ConsentContext<T>>
where
    T: Clone + PartialEq + 'static,
{
    use_context::<ConsentContext<T>>()
}
