use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::components::playground::Playground;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let canvas: web_sys::HtmlCanvasElement = {
        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into()
            .unwrap()
    };
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let bounding_client_rect = canvas.get_bounding_client_rect();

    let playground = Playground::new();

    render(context, playground);

    view! { cx,
        <h1 class="playground_header">{"Score"}</h1>
    }
}

fn render(ctx: CanvasRenderingContext2d, mut playground: Playground) {
    playground.move_playground();
    playground.draw_playground(&ctx);
    request_animation_frame(|| render(ctx, playground))
}
