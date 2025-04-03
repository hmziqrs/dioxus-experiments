use dioxus::prelude::*;

#[component]
pub fn LoginScreen() -> Element {
    rsx! {
        div {
            h1 { "Login" }
            p { "This is a simple example of a Dioxus app." }
        }
    }
}
