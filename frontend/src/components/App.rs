use leptos::*;

use super::Playground::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Playground/>
    }
}
