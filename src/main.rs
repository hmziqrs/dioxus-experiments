#[allow(non_snake_case)]
use dioxus::{document::Document, prelude::*};
use dioxus_router::prelude::*;

mod home;
mod hot_dog;

use home::HomePage;
use hot_dog::DogApp;

static CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
enum Routes {
    #[route("/")]
    HomePage {},

    #[route("/hot_dog")]
    DogApp {},
}

#[component]
pub fn NavBar() -> Element {
    let nav_items = vec![("Home", "/"), ("Hot Dog", "/hot_dog")];

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
                            a {
                                href: item.1,
                                class: "text-white hover:text-gray-300 px-3 py-2 rounded-md text-sm font-medium",
                                "{item.0}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{ href: CSS }
        NavBar {}
        Router::<Routes> {}
    }
}
