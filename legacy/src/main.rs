use dioxus::html::mo::rspace;
#[allow(non_snake_case)]
use dioxus::{document::Document, prelude::*};
use dioxus_router::prelude::*;

mod home;
mod hot_dog;
mod login;
mod nav_bar;
mod profile;
mod server;
mod store;
mod use_form;

use home::HomePage;
use hot_dog::DogApp;
use login::LoginScreen;
use nav_bar::NavBar;
use profile::ProfileScreen;

static CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Routes {
    #[layout(NavBar)]
        #[route("/")]
        HomePage {},

        #[route("/dog_app")]
        DogApp {},

        #[route("/login")]
        LoginScreen {},

        #[route("/profile")]
        ProfileScreen {},

    #[end_layout]
    #[route("/:..segments")]
    NotFoundPage {segments: Vec<String>},
}

#[component]
fn NotFoundPage(segments: Vec<String>) -> Element {
    return rsx!(h1 { "404 not found" });
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{ href: CSS }
        Router::<Routes> {}
    }
}
