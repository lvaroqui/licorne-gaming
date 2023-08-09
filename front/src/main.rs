mod app;
mod components;

mod client_helper;

use app::App;

use leptos::*;

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_web::MakeConsoleWriter;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .without_time()
        .with_writer(MakeConsoleWriter)
        .with_filter(LevelFilter::INFO); // write events to the console

    tracing_subscriber::registry().with(fmt_layer).init(); // Install these as subscribers to tracing events

    mount_to_body(|cx| view! { cx, <App/> })
}
