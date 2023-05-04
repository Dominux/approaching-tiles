use leptos::*;

use crate::components::TilesLine::*;

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    let initial_tiles: Vec<_> = (0..7).map(|id| (id,)).collect();

    let (tiles, _set_tiles) = create_signal(cx, initial_tiles);

    view! { cx,
        <table class="playground">
            <For
                each=tiles
                key=|tile| tile.0
                view=move |cx, _tile| {
                    view! {cx,
                        <TilesLine/>
                    }
                }
            />
        </table>
    }
}
