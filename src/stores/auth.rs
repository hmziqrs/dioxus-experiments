use super::StateFrame;
use dioxus::prelude::*;
use std::sync::OnceLock;

pub struct AuthState {
    pub user: GlobalSignal<Option<User>>,

    pub login_status: GlobalSignal<StateFrame<bool>>,
    pub logout_status: GlobalSignal<StateFrame<bool>>,

    pub signup_status: GlobalSignal<StateFrame<bool>>,
}

#[derive(Debug, Clone)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: i32, name: String, email: String) -> Self {
        User { id, name, email }
    }

    pub fn dev() -> Self {
        User::new(1, "Dev User".to_string(), "dev@example.com".to_string())
    }
}

impl AuthState {
    pub fn new() -> Self {
        AuthState {
            user: GlobalSignal::new(|| None),
            login_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
            logout_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
            signup_status: GlobalSignal::new(|| StateFrame::<bool>::new()),
        }
    }

    pub async fn login(&self, email: String, password: String) {
        self.login_status.write().set_loading(None);

        let response = reqwest::get("http://localhost:3000").await;

        match response {
            Ok(_) => {
                *self.user.write() = Some(User::dev());
                self.login_status.write().set_success(None, None);
            }
            Err(_) => {
                self.login_status.write().set_failed(None);
            }
        }
    }
}

static AUTH_STATE: OnceLock<AuthState> = OnceLock::new();

pub fn use_auth() -> &'static AuthState {
    AUTH_STATE.get_or_init(|| AuthState::new())
}
