use leptos::*;

use super::Tile::*;

#[component]
pub fn TilesLine(cx: Scope) -> impl IntoView {
    let initial_tiles: Vec<_> = (0..7).map(|id| id).collect();

    let (tiles, set_tiles) = create_signal(cx, initial_tiles);

    let remove_tile_by_id =
        move |id| set_tiles.update(move |tiles| tiles.retain(|tile_id| *tile_id != id));

    view! { cx,
        <div class="tiles_column">
            <For
                each=tiles
                key=|tile_id| *tile_id
                view=move |cx, tile_id| {
                    view! {cx,
                        <Tile cover=tile_id.to_string() on_click=move || remove_tile_by_id(tile_id) />
                    }
                }
            />
        </div>
    }
}
