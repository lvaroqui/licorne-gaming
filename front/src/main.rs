mod app;
mod components;

use app::App;

use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
