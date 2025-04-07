use dioxus::{logger::tracing, prelude::*};

use crate::{
    router::{Route, AUTH_ROUTES},
    stores::auth::use_auth,
};

#[component]
pub fn ContainerNavBar() -> Element {
    let route: Route = use_route();
    let is_secure_route = use_hook(|| AUTH_ROUTES.contains(&route));
    let block_render = use_signal(|| is_secure_route.clone());

    let nav = use_navigator();
    let auth = use_auth();
    let user = auth.user.read();
    let init_state = auth.init_status.read();

    // For tracking active route
    let is_active = move |r: Route| -> String {
        if &route == &r {
            "font-bold text-primary border-b-2 border-primary pb-1".into()
        } else {
            "".into()
        }
    };

    use_hook(move || {
        spawn(async move {
            auth.init().await;
        });
    });

    if is_secure_route && user.is_none() && *block_render.read() {
        if init_state.is_loading() || init_state.is_init() {
            return rsx! {
                div {
                    class: "flex flex-col items-center justify-center h-screen",
                    div {
                        class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-primary",
                    }
                    p {
                        class: "mt-4 text-lg",
                        "Loading..."
                    }
                }
            };
        }

        return rsx! {
            div {
                class: "flex flex-col items-center justify-center",
                p { "Please log in to access this page." }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| { nav.push(Route::LoginScreen {}); },
                    "Log In"
                }
            }
        };
    }

    rsx! {
        nav {
            class: "navbar bg-gradient-to-r from-base-300 to-base-200 shadow-md sticky top-0 z-50 py-3",
            div {
                class: "container mx-auto px-4 flex row",
                div {
                    class: "flex-1",
                    Link {
                        to: Route::HomeScreen {},
                        class: "text-2xl font-bold text-primary transition-all duration-300 hover:text-primary-focus",
                        span { class: "mr-1", "🔥" }
                        "Dioxus Experiments"
                    }
                },
                div {
                    class: "flex-none",
                    ul {
                        class: "menu menu-horizontal gap-2 items-center",
                        li {
                            class: format!("transition-all duration-200 hover:scale-105 {}", is_active(Route::HomeScreen {})),
                            onclick: move |_| { nav.push(Route::HomeScreen {}); },
                            div {
                                class: "flex items-center gap-1 px-3 py-2 rounded-lg hover:bg-base-300",
                                "Home"
                                span { class: "text-sm", "🏠" }
                            }
                        },
                        li {
                            class: format!("transition-all duration-200 hover:scale-105 {}", is_active(Route::AboutScreen {})),
                            onclick: move |_| { nav.push(Route::AboutScreen {}); },
                            div {
                                class: "flex items-center gap-1 px-3 py-2 rounded-lg hover:bg-base-300",
                                span { class: "text-sm", "ℹ️" }
                                "About"
                            }
                        },
                        li {
                            class: format!("transition-all duration-200 hover:scale-105 {}", is_active(Route::BlogScreen {})),
                            onclick: move |_| { nav.push(Route::BlogScreen {}); },
                            div {
                                class: format!("transition-all duration-200 hover:scale-105 {}", is_active(Route::LoginScreen {})),
                                span { class: "text-sm", "📝" }
                                "Blog"
                            }
                        },
                        if user.is_some() {
                            li {
                                class: format!("transition-all duration-200 hover:scale-105 {}", is_active(Route::ProfileScreen {})),
                                onclick: move |_| { nav.push(Route::ProfileScreen {}); },
                                div {
                                    class: "flex items-center gap-1 px-3 py-2 rounded-lg hover:bg-base-300",
                                    span { class: "text-sm", "👤" }
                                    "Profile"
                                }
                            }
                        } else {
                            li {
                                onclick: move |_| { nav.push(Route::LoginScreen {}); },
                                div {
                                    class: "flex items-center gap-1 px-3 py-2 bg-primary text-primary-content rounded-lg hover:bg-primary-focus transition-colors duration-200",
                                    span { class: "text-sm", "🔐" }
                                    "Login"
                                }
                            }
                        }
                    }
                }
            }
        }
        Outlet::<crate::router::Route> {}
    }
}
