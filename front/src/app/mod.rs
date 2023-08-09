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
pub fn App(cx: Scope) -> impl IntoView {
    init_auth_state(cx);
    view! { cx,
        <Show
            when=move || auth::Auth::ready(cx).get()
            fallback=|cx| view! { cx, <p>Checking connection...</p> }
        >
            {move || {
                let auth_state_memo = auth::Auth::state_memo(cx);

                // Use memo here as ProtectedRoute does no memoize the result of
                // the condition causing a re-render when auth_state update.
                let logged_in = create_memo(
                    cx,
                    move |_| { auth_state_memo.with(|s| s.logged_in()) },
                );

                view! { cx,
                    <Router>
                        <nav>
                            {move || match auth_state_memo.get() {
                                auth::AuthState::LoggedIn(user) => {
                                    view! { cx, <p>{user.username}</p> }
                                }
                                auth::AuthState::LoggedOut => {
                                    view! { cx, <p>":'("</p> }
                                }
                            }}

                        </nav>
                        <main class="w-screen h-screen bg-gray-300">
                            <Routes>
                                <Route path="/" view=Home/>
                                <ProtectedRoute
                                    path="/login"
                                    view=Login
                                    condition=move |_| !logged_in()
                                    redirect_path="/account"
                                />
                                <ProtectedRoute
                                    path="/register"
                                    view=Register
                                    condition=move |_| !logged_in()
                                    redirect_path="/account"
                                />
                                <ProtectedRoute
                                    path="/account"
                                    view=Account
                                    condition=move |_| logged_in()
                                    redirect_path="/login"
                                />
                            </Routes>
                        </main>
                    </Router>
                }
                    .into_view(cx)
            }}

        </Show>
    }
}
