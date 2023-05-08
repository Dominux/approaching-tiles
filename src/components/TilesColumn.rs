use leptos::*;

use super::Tile::*;
use crate::enums::CheckingResult;

#[component]
pub fn TilesColumn(
    cx: Scope,
    selected_keys: ReadSignal<Vec<u8>>,
    set_selected_keys: WriteSignal<Vec<u8>>,
    checking_result: ReadSignal<CheckingResult>,
    is_add_row: ReadSignal<bool>,
) -> impl IntoView {
    let initial_tiles: Vec<_> = (0..7).map(|id| (id, create_signal(cx, false))).collect();
    let (tiles, set_tiles) = create_signal(cx, initial_tiles);

    create_effect(cx, move |_| match checking_result() {
        CheckingResult::Leave => set_tiles.update(|tiles| {
            tiles
                .iter()
                .for_each(|(_, (_, set_is_selected))| set_is_selected.set(false))
        }),
        CheckingResult::Delete if selected_keys().len() != 0 => {
            log!("{:?}", selected_keys().len());
            set_tiles.update(|tiles| tiles.retain(|(_, (is_selected, _))| !is_selected()))
        }
        _ => (),
    });

    create_effect(cx, move |_| {
        if is_add_row() {
            set_tiles.update(|tiles| {
                let next_id = tiles.last().map_or(0, |(id, _)| id + 1);
                let next_tile = (next_id, create_signal(cx, false));
                tiles.push(next_tile)
            })
        }
    });

    let on_select =
        move |id: u8, is_selected: ReadSignal<bool>, set_is_selected: WriteSignal<bool>| {
            if !is_selected() {
                set_is_selected.update(|v| *v = true);
                set_selected_keys.update(|v| v.push(id))
            }
        };

    view! { cx,
        <div class="tiles_column">
            <For
                each=tiles
                key=|(id, _)| *id
                view=move |cx, (id, (is_selected, set_is_selected))| {
                    view! {cx,
                        <Tile
                            cover=id.to_string()
                            is_selected
                            on_click=move || on_select(id, is_selected, set_is_selected)
                        />
                    }
                }
            />
        </div>
    }
}
