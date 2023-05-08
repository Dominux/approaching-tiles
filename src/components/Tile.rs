use leptos::*;

#[component]
pub fn Tile<F>(
    cx: Scope,
    cover: String,
    is_selected: ReadSignal<bool>,
    on_click: F,
) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! { cx,
        <div
            class="tile"
            class:selected=move || is_selected()
            class:diselected=move || !is_selected()
            on:click=move |_| on_click()
        >
            {cover}
        </div>
    }
}
