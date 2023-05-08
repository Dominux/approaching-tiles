use leptos::*;

#[component]
pub fn Tile<F>(cx: Scope, cover: String, on_click: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! { cx,
        <div
            class="tile"
            on:click=move |_| on_click()
        >
            {cover}
        </div>
    }
}
