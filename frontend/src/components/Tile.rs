use leptos::*;

#[component]
pub fn Tile<F>(cx: Scope, cover: String, is_visible: ReadSignal<bool>, on_click: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! { cx,
        <td
            class="tile"
            class:hidden=move || !is_visible()
            on:click=move |_| on_click()
        >
            {cover}
        </td>
    }
}
