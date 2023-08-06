use leptos::*;
use leptos_router::use_navigate;

use crate::components::Button;
use crate::components::TextInput;

async fn load_data(name: &str) -> String {
    let config = openapi::apis::configuration::Configuration {
        base_path: "http://localhost:8080/api".to_string(),
        ..Default::default()
    };
    let a = openapi::apis::default_api::hello(&config, name)
        .await
        .map(|a| a.name);

    a.unwrap()
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let login = create_rw_signal(cx, String::new());
    let password = create_rw_signal(cx, String::new());
    let navigate = use_navigate(cx);
    let go_to_home = move |_event| {
        navigate("/", Default::default()).unwrap();
    };

    // our resource
    let async_data = create_resource(cx, login, |value| async move { load_data(&value).await });

    view! { cx,
        <div class="flex items-center justify-center w-full h-full">
            <div class="max-sm:container flex flex-col w-full max-w-sm p-3 m-4 border-2 border-gray-900 shadow-xl bg-slate-700 rounded-xl">
                <h1 class="mb-2 text-3xl font-bold text-white">Login</h1>
                <TextInput value=login placeholder="Logian" />
                <TextInput value=password placeholder="Password" password=true />
                <Button on:click=go_to_home>Login</Button>

                <Transition
                    fallback=move || view! { cx, <p>"Loading..."</p> }
                >
                    <h2>"My Data"</h2>
                    {move || {
                        async_data.read(cx)
                    }}
                </Transition>
            </div>
        </div>
    }
}
