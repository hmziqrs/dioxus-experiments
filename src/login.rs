use dioxus::prelude::*;

#[component]
pub fn LoginScreen() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "container flex items-center justify-center",
            div {
                class: "card w-96 border mt-16",
                div {
                    class: "card-body",
                    h1 { class: "card-title", "Login" }
                    div {
                        class: "card-body",
                        form {
                            class: "form",
                            label {
                                class: "input",
                                input {
                                    type: "email",
                                    name: "email",
                                    class: "grow",
                                    placeholder: "Email address",
                                    oninput: move |e| email.set(e.value().clone())
                                }
                            },
                            div { class : "h-4" },
                            label {
                                class: "input",
                                input {
                                    type: "password",
                                    name: "password",
                                    class: "grow",
                                    placeholder: "Password",
                                    oninput: move |e| password.set(e.value().clone())
                                }
                            },
                        }
                        div {
                            class: "card-actions justify-end",
                            button {
                                class: "btn btn-primary",
                                "Login"
                            }
                        }
                    }
                }
            }

        }
    }
}
