use dioxus::prelude::*;

#[component]
pub fn AboutScreen() -> Element {
    rsx! {
        div {
            h1 { "About" }
            p { "This is a simple example of a Dioxus app." }
        }
    }
}
