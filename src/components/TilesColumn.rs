use leptos::*;
use rand::seq::IteratorRandom;

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
    let initial_tiles: Vec<_> = (0..7)
        .rev()
        .map(|id| (id, get_random_key(), create_signal(cx, false)))
        .collect();
    let (tiles, set_tiles) = create_signal(cx, initial_tiles);

    create_effect(cx, move |_| match checking_result() {
        CheckingResult::Leave => set_tiles.update(|tiles| {
            tiles
                .iter()
                .for_each(|(_, _, (_, set_is_selected))| set_is_selected.set(false))
        }),
        CheckingResult::Delete if selected_keys().len() != 0 => {
            log!("{:?}", selected_keys().len());
            set_tiles.update(|tiles| tiles.retain(|(_, _, (is_selected, _))| !is_selected()))
        }
        _ => (),
    });

    create_effect(cx, move |_| {
        if is_add_row() {
            set_tiles.update(|tiles| {
                let next_id = tiles.first().map_or(0, |(id, _, _)| id + 1);
                let next_tile = (next_id, get_random_key(), create_signal(cx, false));
                tiles.insert(0, next_tile)
            })
        }
    });

    let on_select =
        move |key: u8, is_selected: ReadSignal<bool>, set_is_selected: WriteSignal<bool>| {
            if !is_selected() {
                set_is_selected.update(|v| *v = true);
                set_selected_keys.update(|v| v.push(key))
            }
        };

    view! { cx,
        <div class="tiles_column">
            <For
                each=tiles
                key=|(id, _, _)| *id
                view=move |cx, (_, key, (is_selected, set_is_selected))| {
                    view! {cx,
                        <Tile
                            cover=get_symbol(key)
                            is_selected
                            on_click=move || on_select(key, is_selected, set_is_selected)
                        />
                    }
                }
            />
        </div>
    }
}

fn get_random_key() -> u8 {
    (0..8).choose(&mut rand::thread_rng()).unwrap_or_default()
}

fn get_symbol(index: u8) -> String {
    let symbols = ["🪭", "🦕", "🦔", "🐧", "🍑", "🍆", "🥒", "💎"];
    symbols
        .get(index as usize)
        .unwrap_or(&symbols[0])
        .to_string()
}
