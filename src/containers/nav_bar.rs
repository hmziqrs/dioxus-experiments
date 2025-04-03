use dioxus::prelude::*;

use crate::{router::Route, stores::auth::use_auth};

#[component]
pub fn ContainerNavBar() -> Element {
    let nav = use_navigator();
    let auth = use_auth();

    let user = auth.user.read();

    rsx! {
        nav {
            class: "navbar bg-base-200",
            div {
                class: "container flex mx-auto",
                div {
                    class: "flex-1",
                    Link {
                        to: Route::HomeScreen {  },
                        class: "text-2xl font-medium",
                        "Dioxus Experiments"
                    }
                },
                menu {
                    class: "flex-none ",
                    ul {
                        class: "menu menu-horizontal space-x-4 cursor-pointer [&>li]:hover:underline",
                        li {
                            onclick: move |_| { nav.push(Route::HomeScreen {  }); },
                            "Home"
                        },
                        li {
                            onclick: move |_| { nav.push(Route::AboutScreen {  }); },
                            "About"
                        },
                        if user.is_some() {
                            li {
                                onclick: move |_| { nav.push(Route::ProfileScreen {  }); },
                                "Profile"
                            },
                        } else {
                            li {
                                onclick: move |_| { nav.push(Route::LoginScreen {  }); },
                                "Login"
                            },
                        }
                    }
                }
            }
        }
        Outlet::<crate::router::Route> {}
    }
}
