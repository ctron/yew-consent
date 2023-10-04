use yew::prelude::*;
use yew_consent::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AskConsentProperties {
    pub context: ConsentContext,
}

#[function_component(AskConsent)]
pub fn ask_consent(props: &AskConsentProperties) -> Html {
    let onyes = use_callback(props.context.clone(), |_, consent| {
        consent.set(ConsentState::Yes(()))
    });
    let onno = use_callback(props.context.clone(), |_, consent| {
        consent.set(ConsentState::No)
    });

    html!(
        <>
            {"We need your consent: "}
            <button onclick={onyes}>{"I am fine"}</button>
            {" "}
            <button onclick={onno}>{"Please don't"}</button>
        </>
    )
}
