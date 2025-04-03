use dioxus::prelude::*;

#[component]
pub fn NotFoundScreen(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "404 Not Found" }
            p { "The page you are looking for does not exist." }
        }
    }
}
