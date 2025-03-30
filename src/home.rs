use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            h1 { "Welcome to the Home Page!" }
        }
    }
}
