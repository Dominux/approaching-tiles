use leptos::*;

use crate::components::App::*;

mod common;
mod components;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}
