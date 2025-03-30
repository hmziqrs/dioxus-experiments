use std::fs::OpenOptions;

use dioxus::{
    logger::tracing::{self},
    prelude::*,
};

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogApp(breed: String) -> Element {
    let mut img = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

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
                img { src: img.cloned().unwrap_or_default() }
            }
            div { class: "h-4" }
            div {
                class: "space-x-4",
                button {
                    onclick: move |_| img.restart(),
                    class: "btn btn-primary", "Skip",  },
                button {
                    onclick:  move |_| async move {
                        let image = img.cloned().unwrap();
                        img.restart();
                        _ = save_dog(image).await;
                    },
                    class: "btn ", "Save, " }
            }
        }
    }
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}
