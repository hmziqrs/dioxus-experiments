use dioxus::prelude::*;

#[component]
pub fn PostScreen(id: String) -> Element {
    rsx! {
        div {
            h1 { "Post" }
            p { "Welcome to the post!" }
        }
    }
}
