use std::f64;
use std::time::Duration;

use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[component]
pub fn Playground(cx: Scope) -> impl IntoView {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
    let (k, set_k) = create_signal(cx, 2.0);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let view = view! { cx,
        <h1 class="playground_header">{"Score"}</h1>
    };

    let _ = set_interval_with_handle(
        move || {
            draw_smile(&context, k());
            set_k.update(|k| *k -= 0.01)
        },
        Duration::from_millis(17), // ~ 60fps
    );

    view
}

fn draw_smile(ctx: &CanvasRenderingContext2d, k: f64) {
    let pi_k = f64::consts::PI * k;

    // clearing canvas
    {
        let canvas = ctx.canvas().unwrap();
        canvas.set_width(canvas.width())
    }

    ctx.begin_path();

    // Draw the outer circle.
    ctx.arc(75.0, 75.0, 50.0, 0.0, pi_k).unwrap();

    // Draw the mouth.
    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI * k / 2.0)
        .unwrap();

    // Draw the left eye.
    ctx.move_to(65.0, 65.0);
    ctx.arc(60.0, 65.0, 5.0, 0.0, pi_k).unwrap();

    // Draw the right eye.
    ctx.move_to(95.0, 65.0);
    ctx.arc(90.0, 65.0, 5.0, 0.0, pi_k).unwrap();

    ctx.stroke();
}
