use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());

    view! {
        cx,
        <div>
            <h1>"Ashos Leptos WASM!!!"</h1>
        </div>
    }
}
