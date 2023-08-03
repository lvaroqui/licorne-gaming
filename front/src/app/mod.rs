mod home;
mod login;

use home::Home;
use login::Login;

use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <nav>
            </nav>
            <main class="h-screen w-screen bg-gray-200">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/login" view=Login/>
                </Routes>
            </main>
        </Router>
    }
}
