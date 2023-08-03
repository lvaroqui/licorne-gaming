use leptos::*;

use crate::components::TextInput;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex items-center justify-center w-full h-full">
            <div class="flex flex-col bg-gray-600 p-3 rounded-xl  border-1 border-gray-900 shadow-md">
                <h1 class="text-2xl text-white font-bold mb-2">Login</h1>
                <TextInput placeholder="Login" />
                <TextInput placeholder="Password" password=true />
            </div>
        </div>
    }
}
