use dioxus::{logger::tracing, prelude::*};

static DAD: GlobalSignal<i32> = Global::new(|| 0);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MomState {
    daughter_count: i32,
    grand_daughter_count: i32,
}

impl MomState {
    pub fn new() -> Self {
        MomState {
            daughter_count: 0,
            grand_daughter_count: 0,
        }
    }

    fn increment_daughter(&mut self) {
        self.daughter_count += 1;
    }

    fn increment_grand_daughter(&mut self) {
        self.grand_daughter_count += 1;
    }
}

static MOM: GlobalSignal<MomState> = Global::new(|| MomState::new());

#[component]
pub fn HomeScreen() -> Element {
    let dad_count = DAD.read();
    rsx! {
        div {
            class: "container mx-auto p-4",
            h1 {
                class: "text-4xl font-bold mb-4",
                "Home {dad_count}"
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
                onclick: move |_| *DAD.write() += 1,
                "Increment Dad"
            }
            Son {}
        }
    }
}

#[component]
pub fn Son() -> Element {
    tracing::info!("Son component render");
    let dad_count = DAD.read();
    rsx! {
        div {
            class: "card bg-base-100 shadow-xl p-4",
            h2 {
                class: "text-xl font-semibold",
                "Son component: {dad_count}"
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
