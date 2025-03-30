use dioxus::{document::Document, prelude::*};
use guide_component::DogApp;

pub mod guide_component;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{ href: CSS },
        DogApp{
            breed: "LabradorNix"
        }
    }
}
