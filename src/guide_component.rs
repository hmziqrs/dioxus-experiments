use dioxus::{
    logger::tracing::{self},
    prelude::*,
};

#[component]
pub fn DogApp(breed: String) -> Element {
    tracing::info!("Rendered with breed: {breed}");

    rsx! {
        div{
            id: "title",
            h1 { "Hot Dog App" }
        }
        div {
            id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div {
            id: "buttons",
            button { id: "skip", "Skip" },
            button { id: "save", "Save" }
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct DogAppProps {
//     pub breed: String,
// }
