use dioxus::prelude::*;

pub mod components;
pub mod containers;
pub mod hooks;
pub mod router;
pub mod screens;
pub mod stores;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<crate::router::Route> {}
    }
}
