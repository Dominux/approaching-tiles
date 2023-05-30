use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::{
    common::canvas::{canvas_event_listener, get_canvas, get_canvas_coords},
    components::playground::Playground,
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let init_playground = Playground::new();
    let (playground, set_playground) = create_signal(cx, init_playground);

    let ctx = get_canvas()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // setting looped render
    render(ctx, playground, set_playground);

    // listening clicks from user
    canvas_event_listener(ev::click, move |e| {
        let (x, y) = get_canvas_coords(e.x().into(), e.y().into());
        // log!("x: {}; y: {}", x, y);
        set_playground.update(|playground| playground.on_click(x, y))
    });

    view! { cx,
        <h1 class="playground_header">{"Score"}</h1>
    }
}

fn render(
    ctx: CanvasRenderingContext2d,
    playground: ReadSignal<Playground>,
    set_playground: WriteSignal<Playground>,
) {
    set_playground.update(|playground| playground.move_playground());
    playground.get_untracked().draw_playground(&ctx);
    request_animation_frame(move || render(ctx, playground, set_playground))
}
