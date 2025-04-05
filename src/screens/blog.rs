use dioxus::prelude::*;

#[component]
pub fn BlogScreen() -> Element {
    rsx! {
        div {
            h1 { "Blog" }
            p { "Welcome to the blog!" }
        }
    }
}
