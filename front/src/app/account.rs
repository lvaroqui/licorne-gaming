use leptos::*;
use leptos_router::use_navigate;

use crate::{app::auth, components::Button};

#[component]
pub fn Account() -> impl IntoView {
    auth::Auth::ensure_logged_in();

    let logout_action = auth::Auth::logout_action();

    create_effect(move |_| {
        if let Some(_res) = logout_action.value().get() {
            let navigate = use_navigate();
            navigate("/", Default::default());
        }
    });

    view! {
        <div>
            <p>"Congrats, you're in!"</p>
            <Button on:click=move |_cx| { logout_action.dispatch(()) }>Logout</Button>
        </div>
    }
}
