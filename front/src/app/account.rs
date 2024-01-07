use leptos::*;

use crate::{app::auth, components::Button};

#[component]
pub fn Account() -> impl IntoView {
    let logout_action = auth::Auth::logout_action();

    view! {
        <div>
            <p>"Congrats, you're in!"</p>
            <Button on:click=move |_cx| { logout_action.dispatch(()) }>Logout</Button>
        </div>
    }
}
