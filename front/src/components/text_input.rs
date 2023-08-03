use leptos::*;

#[component]
pub fn TextInput(
    cx: Scope,
    #[prop(into)] placeholder: &'static str,
    #[prop(default = false)] password: bool,
) -> impl IntoView {
    let t = if password { "password" } else { "text" };

    view! { cx,
        <input
            class="my-2 p-1 rounded-md outline-none"
            placeholder=placeholder
            type=t
        />
    }
}
