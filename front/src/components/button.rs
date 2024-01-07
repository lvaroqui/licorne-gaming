use leptos::*;

#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <button class="p-1 my-2 transition bg-green-400 border-2 border-gray-900 rounded-md active:bg-green-300 hover:border-white">
            {children()}
        </button>
    }
}
