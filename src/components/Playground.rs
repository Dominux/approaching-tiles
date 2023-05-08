use leptos::*;

use crate::{common::enums::CheckingResult, components::TilesColumn::*};

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    let (selected_keys, set_selected_keys) = create_signal(cx, Vec::<u8>::with_capacity(3));
    let (score, set_score) = create_signal(cx, 0);
    let (checking_result, set_checking_result) = create_signal(cx, CheckingResult::default());

    create_effect(cx, move |_| match selected_keys().len() {
        3 => {
            // checking result
            let mut ch = CheckingResult::Leave;
            if let Some((first, remaining)) = selected_keys().split_first() {
                if remaining.iter().all(|key| *key == *first) {
                    set_score.update(|v| *v += 3);
                    ch = CheckingResult::Delete;
                }
            };

            set_checking_result.update(|v| *v = ch);
            set_selected_keys.update(|keys| keys.clear())
        }
        _ => set_checking_result.set(CheckingResult::default()),
    });

    view! { cx,
        <h1>{"Score: "}{score}</h1>
        <div class="playground">
            {
                (0..7)
                    .map(|_i| view! {cx, <TilesColumn selected_keys set_selected_keys checking_result/>})
                    .collect::<Vec<_>>()
            }
        </div>
    }
}
