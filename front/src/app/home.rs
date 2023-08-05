use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <div>
            <a href="login">Login</a>
        </div>
    }
}
