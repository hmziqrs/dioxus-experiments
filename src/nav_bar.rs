use dioxus::{logger::tracing, prelude::*};

use crate::store::{use_auth_store, Store};

#[component]
pub fn NavBar() -> Element {
    let nav_items = vec![
        ("Home", crate::Routes::HomePage {}),
        ("Hot Dog", crate::Routes::DogApp {}),
        ("Login", crate::Routes::LoginScreen {}),
    ];
    let auth_store = use_auth_store();
    let auth_state = Store::use_store(&auth_store, |s| s.clone());
    let user = auth_state.user;

    tracing::info!("NAVBAR: {user:?}");

    rsx! {
        nav {
            class: "bg-zinc-800",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex items-center justify-between h-16",
                    div {
                        class: "flex items-center",
                        a {
                            href: "/",
                            class: "text-white font-bold text-xl",
                            "Hot Dog"
                        }
                    }
                    div {
                        class: "flex items-center",
                        for item in nav_items {
                            Link {
                                key: item.0,
                                to: item.1,
                                class: "text-white hover:text-gray-300 px-3 py-2 rounded-md text-sm font-medium",
                                if item.0 == "Login" && user.is_some() {
                                    "Logout"
                                } else {
                                    "{item.0}"
                                }
                            }
                        }
                    }
                }
            }
        }
        Outlet::<crate::Routes> {}
    }
}
