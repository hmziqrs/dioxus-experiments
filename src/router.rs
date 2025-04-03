use dioxus::prelude::*;

use crate::containers::ContainerNavBar;
use crate::screens::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(ContainerNavBar)]
    #[route("/")]
    HomeScreen {},

    #[route("/about")]
    AboutScreen {},

    #[route("/login")]
    LoginScreen {},

    #[route("/profile")]
    ProfileScreen {},


    // #[end_layout]

    // #[route("/:..route")]
    // NotFoundScreen { route: Vec<String> }
}
