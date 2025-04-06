use crate::server::{fetch_all_posts, fetch_post_by_id};

// hot_dog/src/stores/posts.rs
use super::StateFrame;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub struct PostsState {
    pub all_posts: GlobalSignal<StateFrame<Vec<Post>>>,
    pub current_post: GlobalSignal<StateFrame<Post>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn new(id: i32, title: String, content: String) -> Self {
        Post { id, title, content }
    }
}

impl PostsState {
    pub fn new() -> Self {
        PostsState {
            all_posts: GlobalSignal::new(|| StateFrame::<Vec<Post>>::new()),
            current_post: GlobalSignal::new(|| StateFrame::<Post>::new()),
        }
    }

    pub async fn fetch_posts(&self) {
        self.all_posts.write().set_loading(None);

        // In a real app, this would be an API call to fetch posts
        let response = fetch_all_posts().await;

        match response {
            Ok(posts) => {
                self.all_posts.write().set_success(Some(posts), None);
            }
            Err(e) => {
                self.all_posts
                    .write()
                    .set_failed(Some(format!("Failed to fetch posts: {}", e)));
            }
        }
    }

    pub async fn fetch_post(&self, id: i32) {
        self.current_post.write().set_loading(None);

        // In a real app, this would be an API call to fetch a specific post
        let response = fetch_post_by_id(id).await;

        match response {
            Ok(post) => {
                self.current_post.write().set_success(Some(post), None);
            }
            Err(e) => {
                self.current_post
                    .write()
                    .set_failed(Some(format!("Failed to fetch post: {}", e)));
            }
        }
    }
}

static POSTS_STATE: OnceLock<PostsState> = OnceLock::new();

pub fn use_posts() -> &'static PostsState {
    POSTS_STATE.get_or_init(|| PostsState::new())
}
