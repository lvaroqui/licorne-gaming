use leptos::*;

#[component]
pub fn TextInput(
    cx: Scope,
    #[prop()] value: RwSignal<String>,
    #[prop(into)] placeholder: MaybeSignal<String>,
    #[prop(into, optional)] password: MaybeSignal<bool>,
) -> impl IntoView {
    let t = if password() { "password" } else { "text" };

    view! { cx,
        <input
            class="px-2 py-1 my-2 border-2 border-gray-900 rounded-md outline-none"
            placeholder=placeholder
            type=t

            on:input=move |ev| {
                value.set(event_target_value(&ev));
            }

            prop:value=value
        />
    }
}
