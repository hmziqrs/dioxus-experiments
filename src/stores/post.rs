use crate::server::{fetch_all_posts, fetch_post_by_id};

// hot_dog/src/stores/posts.rs
use super::StateFrame;
use dioxus::{logger::tracing, prelude::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};

pub struct PostsState {
    pub all_posts: GlobalSignal<StateFrame<Vec<Post>>>,
    pub current_post: GlobalSignal<HashMap<i32, StateFrame<Post>>>,
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
            current_post: GlobalSignal::new(|| HashMap::<i32, StateFrame<Post>>::new()),
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
                tracing::error!("Failed to fetch posts: {}", e);
                self.all_posts
                    .write()
                    .set_failed(Some(format!("Failed to fetch posts: {}", e)));
            }
        }
    }

    pub async fn fetch_post(&self, id: i32) {
        if !self.current_post.peek().contains_key(&id) {
            self.current_post
                .write()
                .insert(id, StateFrame::new_with_loading(None));
        } else {
            self.current_post
                .write()
                .get_mut(&id)
                .unwrap()
                .set_loading(None);
        }

        // In a real app, this would be an API call to fetch a specific post
        let response = fetch_post_by_id(id).await;

        match response {
            Ok(Some(post)) => {
                self.current_post
                    .write()
                    .get_mut(&id)
                    .unwrap()
                    .set_success(Some(post), None);
            }
            Ok(None) => {
                self.current_post
                    .write()
                    .get_mut(&id)
                    .unwrap()
                    .set_failed(Some(format!("Post not found")));
            }
            Err(e) => {
                self.current_post
                    .write()
                    .get_mut(&id)
                    .unwrap()
                    .set_failed(Some(format!("Failed to fetch post: {}", e)));
            }
        }
    }
}

static POSTS_STATE: OnceLock<PostsState> = OnceLock::new();

pub fn use_posts() -> &'static PostsState {
    POSTS_STATE.get_or_init(|| PostsState::new())
}
