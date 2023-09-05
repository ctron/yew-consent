use super::ConsentState;
use gloo_storage::Storage;
use web_sys::console;
use yew::prelude::*;

fn load_state<T>(key: &str) -> Option<ConsentState<T>>
where
    for<'de> T: serde::Deserialize<'de>,
{
    gloo_storage::LocalStorage::get(key).ok()
}

fn store_state<T>(key: &str, state: Option<ConsentState<T>>)
where
    for<'de> T: serde::Serialize,
{
    match state {
        Some(state) => {
            if let Err(err) = gloo_storage::LocalStorage::set(key, state) {
                console::error_2(&"Unable to store state".into(), &err.to_string().into());
            }
        }
        None => gloo_storage::LocalStorage::delete(key),
    }
}

#[derive(Clone, PartialEq)]
pub struct ConsentContext<T = ()> {
    callback: Callback<Option<ConsentState<T>>>,
}

impl<T> ConsentContext<T>
where
    for<'de> T: Clone + PartialEq,
{
    pub fn set(&self, state: impl Into<Option<ConsentState<T>>>) {
        self.callback.emit(state.into());
    }
}

#[derive(PartialEq, Properties)]
pub struct ConsentProperties<T>
where
    T: PartialEq,
{
    #[prop_or("user.consent".into())]
    pub consent_key: AttrValue,

    #[prop_or_default]
    pub children: Children,

    pub ask: Callback<ConsentContext<T>, Html>,
}

#[function_component(Consent)]
pub fn consent<T>(props: &ConsentProperties<T>) -> Html
where
    for<'de> T: Clone + PartialEq + serde::Deserialize<'de> + serde::Serialize + 'static,
{
    let consent = use_state_eq(|| load_state(&props.consent_key));

    let callback = use_callback(
        |state: Option<ConsentState<T>>, (consent_key, consent)| {
            store_state(consent_key, state.clone());
            consent.set(state);
        },
        (props.consent_key.clone(), consent.clone()),
    );
    let context = ConsentContext { callback };

    html!(
        <ContextProvider<ConsentContext<T>> context={context.clone()}>
            {
                match &*consent {
                    Some(state) => {
                        let children: Html = props.children.iter().collect();
                        match &state {
                            ConsentState::Yes(_) => html!(
                                <ContextProvider<ConsentState<T>> context={state.clone()}>
                                    {children}
                                </ContextProvider<ConsentState<T>>>
                            ),
                            ConsentState::No => children,
                        }
                    }
                    None => props.ask.emit(context)
                }
            }
        </ContextProvider<ConsentContext<T>>>
    )
}
