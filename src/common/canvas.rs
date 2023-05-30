use leptos::*;
use wasm_bindgen::prelude::*;

pub fn get_canvas() -> web_sys::HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into()
        .unwrap()
}

fn canvas_event_listener_untyped(event_name: &str, cb: impl Fn(web_sys::Event) + 'static) {
    fn wel(cb: Box<dyn FnMut(web_sys::Event)>, event_name: &str) {
        let cb = Closure::wrap(cb).into_js_value();
        _ = get_canvas().add_event_listener_with_callback(event_name, cb.unchecked_ref());
    }

    wel(Box::new(cb), event_name);
}

pub fn canvas_event_listener<E: ev::EventDescriptor + 'static>(
    event: E,
    cb: impl Fn(E::EventType) + 'static,
) where
    E::EventType: JsCast,
{
    canvas_event_listener_untyped(&event.name(), move |e| {
        cb(e.unchecked_into::<E::EventType>())
    });
}

pub fn get_canvas_coords(x: f64, y: f64) -> (f64, f64) {
    let dom_rect = get_canvas().get_bounding_client_rect();
    (x - dom_rect.x(), y - dom_rect.y())
}

pub fn clear_canvas() {
    let canvas = get_canvas();
    canvas.set_width(canvas.width())
}
