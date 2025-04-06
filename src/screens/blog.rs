use crate::{router::Route, stores::post::use_posts};
use dioxus::prelude::*;

#[component]
pub fn BlogScreen() -> Element {
    let posts = use_posts();
    let posts_state = posts.all_posts.read();
    let posts_list = posts_state.data.clone();

    use_effect(move || {
        spawn(async move {
            posts.fetch_posts().await;
        });
    });

    rsx! {
        div {
            class: "flex flex-col items-center min-h-screen bg-base-200/30 p-8",
            div {
                class: "w-full max-w-4xl space-y-6",
                h1 {
                    class: "text-3xl font-bold text-center text-primary mb-8",
                    "Blog Posts"
                }

                if posts_state.is_loading() && posts_state.data.is_none() {
                    div {
                        class: "flex justify-center items-center h-40",
                        div { class: "loading loading-spinner loading-lg text-primary" }
                    }
                } else if posts_state.is_failed() {
                    div {
                        class: "alert alert-error",
                        span {
                            class: "text-error-content",
                            "Failed to fetch posts"
                            {posts_state.message.as_ref().unwrap().to_string()}
                        }
                    }
                } else if posts_state.data.is_some() {
                    div {
                        class: "posts space-y-4",
                        for post in posts_list.unwrap() {
                            Link {
                                class: "block",
                                to: Route::PostScreen { id: post.id.to_string() },
                                h2 {
                                    class: "text-2xl mb-2",
                                    "{post.title}"
                                }
                                p {
                                    class: "text-sm text-primary/80",
                                    "{post.content}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
