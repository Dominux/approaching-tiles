use leptos::*;

use crate::components::TilesLine::*;

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="playground">
            {
                (0..7).map(|_i| view! {cx, <TilesLine/>}).collect::<Vec<_>>()
            }
        </div>
    }
}
