use leptos::*;
use leptos_router::use_navigate;

use crate::components::Button;
use crate::components::TextInput;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let login = create_rw_signal(cx, String::new());
    let password = create_rw_signal(cx, String::new());
    let navigate = use_navigate(cx);
    let go_to_home = move |_event| {
        navigate("/", Default::default()).unwrap();
    };
    view! { cx,
        <div class="flex items-center justify-center w-full h-full">
            <div class="flex flex-col w-full max-w-sm p-3 m-4 border-2 border-gray-900 shadow-xl bg-slate-700 max-sm:container rounded-xl">
                <h1 class="mb-2 text-3xl font-bold text-white">Login</h1>
                <TextInput value=login placeholder="Login" />
                <TextInput value=password placeholder="Password" password=true />
                <Button on:click=go_to_home>Login</Button>
            </div>
        </div>
    }
}
