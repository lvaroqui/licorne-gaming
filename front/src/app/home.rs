use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
      <div class="bg-blue-500">
        <h1>Hello from Home</h1>
      </div>
    }
}
