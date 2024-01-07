mod account;
mod home;

mod auth;

use account::Account;
use auth::{Login, Register};
use home::Home;

use leptos::*;
use leptos_router::*;

use auth::init_auth_state;
#[component]
pub fn App() -> impl IntoView {
    init_auth_state();

    view! {
        <Router>
            <nav>
                {move || match auth::Auth::state().get() {
                    Some(auth::AuthState::LoggedIn(user)) => {
                        view! {  <p>{user.username}</p> }.into_view()
                    }
                    Some(auth::AuthState::LoggedOut) => {
                        view! {  <p>":'("</p> }.into_view()
                    }
                    None => {
                        view! { }.into_view()
                    }
                }}
            </nav>
            <main class="w-screen h-screen bg-gray-300">
                <Routes>
                    <Route path="/" view=|| view! {
                        <Show when=|| auth::Auth::state().get().is_some() fallback=|| view! { <p>"Loading..."</p> }>
                            <Outlet/>
                        </Show>
                    }>
                        <Route path="/" view=Home/>
                        <Route
                            path="/login"
                            view=Login
                        />
                        <Route
                            path="/register"
                            view=Register
                        />
                        <Route
                            path="/account"
                            view=Account
                        />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
    .into_view()
}
