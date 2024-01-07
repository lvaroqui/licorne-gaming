use leptos::*;
use leptos_router::use_navigate;
use openapi::models::LoginRequest;

use crate::app::auth;
use crate::components::Button;
use crate::components::TextInput;

#[component]
pub fn Login() -> impl IntoView {
    let username = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());

    let login_action = auth::Auth::login_action();

    view! {
        <div class="flex items-center justify-center w-full h-full">
            <form
                class="max-sm:container flex flex-col w-full max-w-sm p-3 m-4 border-2 border-gray-900 shadow-xl bg-slate-700 rounded-xl"
                on:submit=move |ev| {
                    ev.prevent_default();
                    login_action
                        .dispatch(LoginRequest {
                            username: username(),
                            password: password(),
                        })
                }
            >

                <h1 class="mb-2 text-3xl font-bold text-white">Login</h1>
                <TextInput value=username placeholder="Username"/>
                <TextInput value=password placeholder="Password" password=true/>
                <div class="grid grid-cols-1 gap-2 md:grid-cols-2">
                    <Button on:click=move |e| {
                        e.prevent_default();
                        let navigate = use_navigate();
                        navigate("/register", Default::default());
                    }>Register</Button>
                    <Button>Login</Button>
                </div>
            </form>
        </div>
    }
}
