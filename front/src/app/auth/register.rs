use leptos::*;
use leptos_router::use_navigate;
use openapi::models::RegisterRequest;

use crate::app::auth;
use crate::components::Button;
use crate::components::TextInput;

#[component]
pub fn Register() -> impl IntoView {
    auth::Auth::ensure_logged_out();

    let username = create_rw_signal(String::new());
    let email = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let register_action = auth::Auth::register_action();

    create_effect(move |_| {
        if let Some(_res) = register_action.value().get() {
            let navigate = use_navigate();
            navigate("/login", Default::default());
        }
    });

    view! {
        <div class="flex items-center justify-center w-full h-full">
            <form
                class="max-sm:container flex flex-col w-full max-w-sm p-3 m-4 border-2 border-gray-900 shadow-xl bg-slate-700 rounded-xl"
                on:submit=move |e| {
                    e.prevent_default();
                    register_action
                        .dispatch(RegisterRequest {
                            username: username(),
                            email: email(),
                            password: password(),
                        })
                }
            >
                <h1 class="mb-2 text-3xl font-bold text-white">Register</h1>
                <TextInput value=username placeholder="Username"/>
                <TextInput value=email placeholder="Email"/>
                <TextInput value=password placeholder="Password" password=true/>
                <Button>Register</Button>
            </form>
        </div>
    }
}
