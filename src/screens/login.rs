use dioxus::{logger::tracing, prelude::*};

use crate::{
    hooks::use_previous,
    screens::login_form::{use_login_form, LoginForm},
    stores::auth::use_auth,
};

#[component]
pub fn LoginScreen() -> Element {
    let mut ox_form = use_login_form(LoginForm::new());
    let auth = use_auth();
    let login_status = auth.login_status.read();
    let is_loading = login_status.is_loading();
    let prev_loading = use_previous(is_loading);

    let ox_form_state = ox_form.read();
    let email_error = ox_form_state.get_field("email").unwrap().error.clone();

    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen bg-base-200/30",
            div {
                class: "w-full max-w-md p-8 space-y-3 rounded-xl bg-base-100 shadow-2xl",
                h1 {
                    class: "text-2xl font-bold text-center text-primary",
                    "Login"
                }
                form {
                    class: "space-y-6",
                    div {
                        class: "space-y-1",
                        label {
                            class: "block text-sm font-medium text-primary",
                            "Email"
                        }
                        input {
                            type: "email",
                            class: "w-full px-4 py-2 input",
                            placeholder: "Enter your email",
                            value: ox_form_state.get_field("email").unwrap().value.to_owned(),
                            onchange: move |event| {
                                ox_form.write().update_field("email", event.value());
                            },
                            onblur: move |_| {
                                ox_form.write().blur_field("email");
                            },
                            onfocus: move |_| {
                                ox_form.write().focus_field("email");
                            }
                        }
                        if ox_form_state.get_field("email").unwrap().has_error() {
                            p {
                                class: "text-sm text-error",
                                {email_error.unwrap()}
                            }
                        }
                    }
                    div {
                        class: "space-y-1",
                        label {
                            class: "block text-sm font-medium text-primary",
                            "Password"
                        }
                        input {
                            class: "w-full px-4 py-2 input",
                            type: "password",
                            placeholder: "Enter your password",
                        }
                    }
                    div {
                        class: "flex items-center justify-between",
                        label {
                            class: "flex items-center",
                            input {
                                class: "checkbox checkbox-primary",
                                r#type: "checkbox",
                            }
                            span {
                                class: "ml-2 text-sm text-primary",
                                "Remember me"
                            }
                        }
                        a {
                            class: "text-sm text-primary hover:underline",
                            href: "#",
                            "Forgot password?"
                        }
                    }
                    div {
                        class: "w-full  btn btn-primary",
                        onclick: move |_| {
                            spawn(async move {
                                auth.login("hi".to_owned(), "to".to_owned()).await;
                            });
                        },
                        "Login"
                    }
                }
                p {
                    class: "text-sm text-center text-primary",
                    "Don't have an account? "
                    a {
                        class: "text-primary hover:underline",
                        href: "#",
                        "Sign up"
                    }
                }
            }
        }
    }
}
