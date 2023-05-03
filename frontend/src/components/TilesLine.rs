use leptos::*;

#[component]
pub fn TilesLine(cx: Scope) -> impl IntoView {
    let initial_tiles: Vec<_> = (0..7).map(|id| (id,)).collect();

    let (tiles, _set_tiles) = create_signal(cx, initial_tiles);

    view! { cx,
        <For
            each=tiles
            key=|tile| tile.0
            view=move |cx, tile| {
                view! {cx,
                    <span>{tile.0}</span>
                }
            }
        />
    }
}