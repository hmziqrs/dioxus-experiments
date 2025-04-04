use dioxus::{logger::tracing, prelude::*};

use crate::{
    components::{AppInput, AppInputProps},
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
                    AppInput {
                        name: "email",
                        form: ox_form,
                        label: "Email",
                        placeholder: "Please input your email",
                    }
                    AppInput {
                        name: "password",
                        form: ox_form,
                        label: "Password",
                        placeholder: "Please input your password",
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
                            ox_form.write().on_submit(move |val| {
                                tracing::info!("{:?}",val);
                                spawn(async move {
                                    auth.login("hi".to_owned(), "to".to_owned()).await;
                                });
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
