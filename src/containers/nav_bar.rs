use dioxus::prelude::*;

#[component]
pub fn ContainerNavBar() -> Element {
    rsx! {
        nav {
            h1 {
                "Dioxus Experiments"
            }
        }
        Outlet::<crate::router::Route> {}
    }
}
