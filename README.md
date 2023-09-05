# yew-consent

[![crates.io](https://img.shields.io/crates/v/yew-consent.svg)](https://crates.io/crates/yew-consent)
[![docs.rs](https://docs.rs/yew-consent/badge.svg)](https://docs.rs/yew-consent)

> Managing user consent with Yew

## Usage

Add it to your project:

```shell
cargo add yew-consent
```

Then, add it your application component:

```rust
#[function_component(Application)]
fn application() -> Html {
    let ask = use_callback(|context, ()| html!(<AskConsent {context} />), ());

    html!(
        <Consent<()> {ask}>
            <State/>
        </Consent<()>>
    )
}
```

And then, you can check consent later:

```rust
#[function_component(State)]
fn state() -> Html {
    let consent = use_consent::<()>();

    html!(
        <>
            <dl>
                <dt>{"Consent state"}</dt>
                <dd>{ format!("{consent:#?}")}</dd>
            </dl>
        </>
    )
}
```

Also see the example here: [example](example).

You can run the example using:

```shell
cd example
trunk serve
```
