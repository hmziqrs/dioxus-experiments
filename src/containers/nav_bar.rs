use dioxus::{html::u::class, prelude::*};

use crate::router::Route;

#[component]
pub fn ContainerNavBar() -> Element {
    let mut nav = use_navigator();

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
                        li {
                            onclick: move |_| { nav.push(Route::LoginScreen {  }); },
                            "Login"
                        },
                    }
                }
            }
        }
        Outlet::<crate::router::Route> {}
    }
}
