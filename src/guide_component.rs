use dioxus::{
    logger::tracing::{self},
    prelude::*,
};

#[component]
pub fn DogApp(breed: String) -> Element {
    tracing::info!("Rendered with breed: {breed}");

    rsx! {
        div {
            class: "flex flex-col items-center justify-center",
            div { class: "h-4" }
            h1{
                class: "text-center text-2xl font-bold",
                h1 { "Hot Dog App" }
            }
            div { class: "h-4" }
            div {
                id: "dogview",
                class: "w-md",
                img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
            }
            div { class: "h-4" }
            div {
                class: "space-x-4",
                button { class: "btn btn-primary", "Skip" },
                button { class: "btn ", "Save" }
            }
        }
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct DogAppProps {
//     pub breed: String,
// }
