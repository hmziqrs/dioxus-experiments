use dioxus::{html::textarea::disabled, logger::tracing, prelude::*};

use crate::store::{use_auth_actions, use_auth_store, AuthState, Store};

#[component]
pub fn LoginScreen() -> Element {
    let mut email = use_signal(|| "user@example.com".to_string());
    let mut password = use_signal(|| "password".to_string());
    let auth_store = use_auth_store();
    let auth_state = Store::use_store(&auth_store, |s| s.clone());
    let auth_actions = use_auth_actions(&auth_store);

    let loading = auth_state.login_status.is_loading();

    tracing::info!("{email} {password} {loading:?}");

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
                                    value: email.read().clone(),
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
                                    value: password.read().clone(),
                                    type: "password",
                                    name: "password",
                                    class: "grow",
                                    placeholder: "Password",
                                    oninput: move |e| password.set(e.value().clone())
                                }
                            },
                        }
                        if loading {
                                div {
                                    class: "loading loading-spinner loading-lg",
                                }
                        }
                        div {
                            class: "card-actions justify-end",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    // Clone the values we need to pass to the async block
                                    let email_value = email.read().clone();
                                    let password_value = password.read().clone();
                                    let actions = auth_actions.clone();

                                    spawn(async move {
                                        tracing::info!("login clicked");
                                        actions.login(&email_value, &password_value).await;
                                    });

                                },
                                "Login",
                            }
                        }
                    }
                }
            }

        }
    }
}
