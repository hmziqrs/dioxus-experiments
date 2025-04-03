use dioxus::{logger::tracing, prelude::*};

use crate::store::{use_auth_store, Store};

#[component]
pub fn ProfileScreen() -> Element {
    let auth_store = use_auth_store();
    let auth_state = Store::use_store(&auth_store, |s| s.clone());
    let user = auth_state.user;

    tracing::info!("{user:?}");
    return rsx! {
        div {
            h1 { "Profile" }
            p { "This is your profile page." }
        }
    };
}
