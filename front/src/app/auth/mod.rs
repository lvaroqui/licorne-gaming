use leptos::*;
use leptos_router::use_navigate;
use openapi::models::{LoginRequest, RegisterRequest, User};

use crate::client_helper::client_config;

mod login;
mod register;

pub use login::Login;
pub use register::Register;

#[derive(Debug, Clone, PartialEq)]
pub enum AuthState {
    LoggedIn(User),
    LoggedOut,
}

impl AuthState {
    pub fn logged_in(&self) -> bool {
        matches!(self, AuthState::LoggedIn(_))
    }
}

#[derive(Clone, Copy)]
pub struct Auth {
    state: RwSignal<Option<AuthState>>,
}

impl Auth {
    pub fn state(
    ) -> impl SignalGet<Value = Option<AuthState>> + SignalWith<Value = Option<AuthState>> {
        state().state
    }

    pub fn ensure_logged_in() {
        if !Self::state().get().unwrap().logged_in() {
            let navigate = use_navigate();
            navigate("/login", Default::default());
        }
    }

    pub fn ensure_logged_out() {
        if Self::state().get().unwrap().logged_in() {
            let navigate = use_navigate();
            navigate("/account", Default::default());
        }
    }

    pub fn register_action() -> Action<RegisterRequest, ()> {
        create_action(move |r: &RegisterRequest| {
            let r = r.clone();
            async move {
                openapi::apis::default_api::register(client_config(), r)
                    .await
                    .unwrap();
            }
        })
    }

    pub fn login_action() -> Action<LoginRequest, AuthState> {
        create_action(move |r: &LoginRequest| {
            let r = r.clone();
            async move {
                let s = match openapi::apis::default_api::login(client_config(), r).await {
                    Ok(user) => AuthState::LoggedIn(user),
                    Err(_) => AuthState::LoggedOut,
                };
                state().state.set(Some(s.clone()));
                s
            }
        })
    }

    pub fn logout_action() -> Action<(), ()> {
        create_action(move |_| async move {
            openapi::apis::default_api::logout(client_config())
                .await
                .unwrap();
            state().state.set(Some(AuthState::LoggedOut));
        })
    }
}

pub fn init_auth_state() {
    let me_action = create_action(move |_| async move {
        let s = match openapi::apis::default_api::me(client_config()).await {
            Ok(user) => AuthState::LoggedIn(user),
            Err(_) => AuthState::LoggedOut,
        };
        state().state.set(Some(s));
    });

    provide_context(Auth {
        state: create_rw_signal(None),
    });

    me_action.dispatch(());
}

pub fn state() -> Auth {
    expect_context::<Auth>()
}
