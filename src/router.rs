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

    #[route("/blog")]
    BlogScreen {},

    #[route("/post/:id")]
    PostScreen { id: i32 },

    // #[end_layout]

    #[route("/:..route")]
    NotFoundScreen { route: Vec<String> }
}

pub static AUTH_ROUTES: std::sync::LazyLock<Vec<Route>> =
    std::sync::LazyLock::new(|| vec![Route::ProfileScreen {}]);
