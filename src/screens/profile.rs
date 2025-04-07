use dioxus::prelude::*;

use crate::{router::Route, stores::auth::use_auth};

#[component]
pub fn ProfileScreen() -> Element {
    let auth = use_auth();
    let user = auth.user.read();

    let is_logged_in = user.is_some();

    if is_logged_in {
        rsx! {
            div {
                h1 { "Profile" },
                p { "Welcome: {user:?}" },
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        spawn(async move {
                            auth.logout().await;
                        });
                    },
                    "Logout",
                }
            }
        }
    } else {
        rsx! {
            div {
                h1 { "Login" },
                p { "Please log in to view your profile." },
                Link {
                    to: Route::LoginScreen {  },
                    "Login"
                }
            }
        }
    }
}
