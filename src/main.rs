use leptos::*;

use crate::components::App::*;

mod components;
mod enums;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}
