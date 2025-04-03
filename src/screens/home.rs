use dioxus::prelude::*;

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div {
            h1 { "Home" }
            p { "This is a simple example of a Dioxus app." }
        }
    }
}
