use std::rc::Rc;

use dioxus::{
    html::{audio::autoplay, form::action},
    logger::tracing,
    prelude::*,
};

use crate::store::{use_auth_store, Store};

#[component]
pub fn NavBar() -> Element {
    let nav_items = vec![
        ("Home", crate::Routes::HomePage {}),
        ("Hot Dog", crate::Routes::DogApp {}),
        ("Login", crate::Routes::LoginScreen {}),
        ("Profile", crate::Routes::ProfileScreen {}),
    ];
    let auth_store = use_auth_store();
    // let rc =
    let auth_state = Store::use_store(&auth_store, |s| s.clone());

    let mut nav = use_navigator();
    let user = auth_state.user;
    let is_logged_in = user.is_some();

    rsx! {
        nav {
            class: "bg-zinc-800",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex items-center justify-between h-16",
                    div {
                        class: "flex items-center",
                        Link {
                            to: crate::Routes::HomePage {  },
                            class: "text-white font-bold text-xl",
                            "Hot Dog"
                        }
                    }
                    button {
                        onclick: move |_| {
                            tracing::info!("Logout");
                            let actions = auth_store.clone();
                            spawn(async move {
                                // actions.logout().await;
                            });
                            tracing::info!("Logout-ppost");
                        },
                        class: "text-white hover:text-gray-300 px-3 py-2 rounded-md text-sm font-medium",
                        "Logout-nnn"
                    }
                    div {
                        class: "flex items-center",
                        for item in nav_items {
                            button {
                                // key: item.0,
                                onclick: move |_| {

                                    tracing::info!("{{item.0}}");
                                    if item.0 == "Login" && is_logged_in {
                                    } else {
                                        nav.push(item.1.clone());
                                    }
                                },
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
