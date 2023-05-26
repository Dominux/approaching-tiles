use leptos::*;

use crate::App::*;

#[allow(non_snake_case)]
mod App;
mod components;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}
