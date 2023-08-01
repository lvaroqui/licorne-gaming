mod home;

use home::Home;

use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
        <Routes>
          <Route path="/" view=Home/>
        </Routes>
        </main>
      </Router>
    }
}
