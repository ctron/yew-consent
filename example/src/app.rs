use crate::components::AskConsent;
use yew::prelude::*;
use yew_consent::prelude::*;

#[function_component(Application)]
pub fn application() -> Html {
    let ask = use_callback((), |context, ()| html!(<AskConsent {context} />));

    html!(
        <Consent<()> {ask}>
            <State/>
        </Consent<()>>
    )
}

#[function_component(State)]
fn state() -> Html {
    let consent = use_consent::<()>();
    let consent_context = use_consent_context::<()>();

    let onclear = use_callback(consent_context, |_, context| {
        if let Some(context) = &context {
            context.set(None);
        }
    });

    html!(
        <>
            <dl>
                <dt>{"Consent state"}</dt>
                <dd>{ format!("{consent:#?}")}</dd>
            </dl>

            <button onclick={onclear}>{"Clear state"}</button>
        </>
    )
}
