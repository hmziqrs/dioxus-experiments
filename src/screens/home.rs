use std::sync::OnceLock;

use dioxus::{logger::tracing, prelude::*};

// #[derive(Sync, Clone)]
struct DadState {
    son_count: GlobalSignal<i32>,
    grand_son_count: GlobalSignal<i32>,
    grand_son2_count: GlobalSignal<i32>,
}
impl DadState {
    pub fn new() -> Self {
        DadState {
            son_count: Global::new(|| 0),
            grand_son_count: Global::new(|| 0),
            grand_son2_count: Global::new(|| 0),
        }
    }

    fn increment_son(&self) {
        *self.son_count.write() += 1;
    }

    fn increment_grand_son(&self) {
        *self.grand_son_count.write() += 1;
    }

    fn increment_grand_son2(&self) {
        *self.grand_son2_count.write() += 1;
    }
}

// use std::sync::Once;

static DAD: OnceLock<DadState> = OnceLock::new();

fn dad_state() -> &'static DadState {
    DAD.get_or_init(|| DadState::new())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MomState {
    daughter_count: i32,
    grand_daughter_count: i32,
    grand_daughter2_count: Signal<i32>,
}

impl MomState {
    pub fn new() -> Self {
        MomState {
            daughter_count: 0,
            grand_daughter_count: 0,
            grand_daughter2_count: Signal::new(0),
        }
    }

    fn increment_daughter(&mut self) {
        self.daughter_count += 1;
    }

    fn increment_grand_daughter(&mut self) {
        self.grand_daughter_count += 1;
    }

    fn increment_grand_daughter2(&mut self) {
        let mut s = self.grand_daughter2_count;
        *s.write() += 1;
    }
}

static MOM: GlobalSignal<MomState> = Global::new(|| MomState::new());

#[component]
pub fn HomeScreen() -> Element {
    rsx! {
        div {
            class: "container mx-auto p-4",
            h1 {
                class: "text-4xl font-bold mb-4",
                "Home"
            }
            p {
                class: "text-lg mb-4",
                "This is a simple example of a Dioxus app."
            }
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                Mom {}
                Dad {}
            }
        }
    }
}

#[component]
pub fn Mom() -> Element {
    tracing::info!("Mom component render");
    rsx! {
        div {
            class: "card bg-base-300 shadow-xl p-4",
            h2 {
                class: "text-2xl font-semibold mb-2",
                "Mom"
            }
            button {
                class: "btn btn-primary mb-4",
                onclick: move |_| MOM.write().increment_daughter(),
                "Increment Daughter"
            },
            button {
                class: "btn btn-primary mb-4",
                onclick: move |_| MOM.write().increment_grand_daughter(),
                "Increment Grand Daughter"
            }
            button {
                class: "btn btn-primary mb-4",
                onclick: move |_| {
                    MOM.write().increment_grand_daughter2();
                },
                "Increment Grand Daughter2"
            },
            Daughter {  }
        }
    }
}

#[component]
pub fn Dad() -> Element {
    tracing::info!("Dad component render");
    rsx! {
        div {
            class: "card bg-base-300 shadow-xl p-4",
            h2 {
                class: "text-2xl font-semibold mb-2",
                "Dad component"
            }
            button {
                class: "btn btn-primary mb-4",
                onclick: move |_| dad_state().increment_son(),
                "Increment Dad"
            }
            button {
                class: "btn btn-primary mb-4",
                onclick: move |_| dad_state().increment_grand_son(),
                "Increment Grand Son"
            }
            button {
                class: "btn btn-primary mb-4",
                onclick: move |_| dad_state().increment_grand_son2(),
                "Increment Grand Son2"
            }
            Son {}
        }
    }
}

#[component]
pub fn Son() -> Element {
    tracing::info!("Son component render");
    let dad_count = dad_state().son_count.read();
    rsx! {
        div {
            class: "card bg-base-100 shadow-xl p-4",
            h2 {
                class: "text-xl font-semibold",
                "Son component: {dad_count}"
            }
            GrandSon{}
            GrandSon2{}
        }
    }
}

#[component]
pub fn GrandSon() -> Element {
    tracing::info!("GrandSon component render");
    let dad_count = dad_state().grand_son_count.read();
    rsx! {
        div {
            class: "card bg-base-100 shadow-xl p-4",
            h2 {
                class: "text-xl font-semibold",
                "GrandSon component: {dad_count}"
            }
        }
    }
}

#[component]
pub fn GrandSon2() -> Element {
    tracing::info!("GrandSon2 component render");
    let dad_count = dad_state().grand_son2_count.read();
    rsx! {
        div {
            class: "card bg-base-100 shadow-xl p-4",
            h2 {
                class: "text-xl font-semibold",
                "GrandSon2 component: {dad_count}"
            }
        }
    }
}

#[component]
pub fn Daughter() -> Element {
    tracing::info!("Daughter component render");
    let mom_count = MOM.read().daughter_count;
    rsx! {
        div {
            class: "card bg-base-100 shadow-xl p-4",
            h2 {
                class: "text-xl font-semibold",
                "Daughter component: {mom_count}"
            },
            GrandDaughter {}
            GrandDaughter2 {}
        }
    }
}

#[component]
pub fn GrandDaughter() -> Element {
    tracing::info!("GrandDaughter component render");
    let mom_count = MOM.read().grand_daughter_count;
    rsx! {
        div {
            class: "card bg-base-100 shadow-xl p-4",
            h2 {
                class: "text-xl font-semibold",
                "GrandDaughter component: {mom_count}"
            }
        }
    }
}

#[component]
pub fn GrandDaughter2() -> Element {
    tracing::info!("GrandDaughter2 component render");
    let r = MOM.peek();
    let mom_count = r.grand_daughter2_count.read();
    rsx! {
        div {
            class: "card bg-base-100 shadow-xl p-4",
            h2 {
                class: "text-xl font-semibold",
                "GrandDaughter2 component: {mom_count}"
            }
        }
    }
}
