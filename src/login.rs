use dioxus::{html::textarea::disabled, logger::tracing, prelude::*};

use crate::{
    store::{use_auth_actions, use_auth_store, AuthState, Store},
    use_form::{use_form, LoginForm},
};

#[component]
pub fn LoginScreen() -> Element {
    let mut form = use_form(LoginForm::new());

    let auth_store = use_auth_store();
    let auth_state = Store::use_store(&auth_store, |s| s.clone());
    let auth_actions = use_auth_actions(&auth_store);

    let loading = auth_state.login_status.is_loading();

    // tracing::info!("{r:?}");

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
                                    value: form.read().get_field("email").unwrap().value.clone(),
                                    type: "email",
                                    name: "email",
                                    class: "grow",
                                    placeholder: "Email address",
                                    oninput: move |e| {
                                        let mut form_data = form.write();
                                        form_data.update_field("email", &e.value());
                                    },
                                    onfocusin: move |_| {
                                        let mut form_data = form.write();
                                        form_data.focus_field("email");
                                    },
                                    onfocusout: move |_| {
                                        let mut form_data = form.write();
                                        form_data.blur_field("email");
                                    }
                                }
                            },
                            div { class : "h-4" },
                            label {
                                class: "input",
                                input {
                                    value: form.read().get_field("password").unwrap().value.clone(),
                                    type: "password",
                                    name: "password",
                                    class: "grow",
                                    placeholder: "Password",
                                    oninput: move |e| {
                                        let mut form_data = form.write();
                                        form_data.update_field("password", &e.value());
                                    }
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
                                    let email = form.read().get_field("email").unwrap().value.clone();
                                    let password = form.read().get_field("password").unwrap().value.clone();
                                    let actions = auth_actions.clone();

                                    spawn(async move {
                                        tracing::info!("login clicked");
                                        actions.login(email, password).await;
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
