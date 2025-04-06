use crate::{
    server::fetch_post_by_id,
    stores::{post::use_posts, StateFrame},
};
use dioxus::prelude::*;

#[component]
pub fn PostScreen(id: i32) -> Element {
    let nav = use_navigator();
    let posts = use_posts();
    let post_ref = posts.current_post.read();
    let server_cache = use_server_future(move || fetch_post_by_id(id))?;
    let server_peek = server_cache.peek();
    let post_state = if let Ok(Some(cached)) = server_peek.as_ref().unwrap() {
        StateFrame::new_with_data(Some(cached.clone()))
    } else {
        post_ref.get(&id).cloned().unwrap_or_default()
    };

    let post_success = post_state.is_success();

    use_effect(move || {
        if post_success {
            return;
        }
        spawn(async move {
            _ = posts.fetch_post(id).await;
        });
    });

    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen bg-base-200/30",
            div {
                class: "w-full max-w-2xl p-8 space-y-4 rounded-xl bg-base-100 shadow-2xl",
                if post_state.is_loading() && post_state.data.is_none() {
                    div {
                        class: "flex justify-center items-center h-40",
                        div { class: "loading loading-spinner loading-lg text-primary" }
                    }
                } else if post_state.is_failed() {
                    div {
                        class: "alert alert-error",
                        // span { "{error}" }
                    }
                } else if let Some(post) = post_state.data.clone() {
                    div {
                        class: "space-y-4",
                        div {
                            class: "flex justify-between items-center",
                            h1 {
                                class: "text-3xl font-bold text-primary",
                                "{post.title}"
                            }
                            span {
                                class: "badge badge-primary",
                                "ID: {post.id}"
                            }
                        }
                        div {
                            class: "divider"
                        }
                        div {
                            class: "prose max-w-none",
                            p { "{post.content}" }
                        }
                        div {
                            class: "flex justify-end mt-6 pt-4 border-t",
                            button {
                                class: "btn btn-outline btn-primary",
                                onclick: move |_| {
                                    nav.go_back();
                                },
                                "Back to Posts"
                            }
                        }
                    }
                }
            }
        }
    }
}
