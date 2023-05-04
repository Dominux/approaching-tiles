use leptos::*;

use super::Tile::*;

#[component]
pub fn TilesLine(cx: Scope) -> impl IntoView {
    let tiles = (0..7).map(|_| create_signal(cx, true));

    view! { cx,
        <tr>
            {
                tiles.map(|(tile, set_tile)| {
                    view! {cx,
                        <Tile cover="lol".to_string() is_visible=tile on_click=move || set_tile.set(false) />
                    }
                }).collect::<Vec<_>>()
            }
        </tr>
    }
}
