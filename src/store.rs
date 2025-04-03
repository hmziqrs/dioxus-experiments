use async_std::task;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StateStatus {
    Init,
    Loading,
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateSlice<T: Clone> {
    pub status: StateStatus,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Clone> StateSlice<T> {
    pub fn new() -> Self {
        Self {
            status: StateStatus::Init,
            data: None,
            message: None,
        }
    }

    pub fn is_init(&self) -> bool {
        self.status == StateStatus::Init
    }

    pub fn is_loading(&self) -> bool {
        self.status == StateStatus::Loading
    }

    pub fn is_success(&self) -> bool {
        self.status == StateStatus::Success
    }

    pub fn is_failed(&self) -> bool {
        self.status == StateStatus::Failed
    }
}

type SubscriberId = usize;
static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// The core store structure that maintains state and subscribers
pub struct Store<T: Clone + 'static> {
    state: RefCell<T>,
    // subscribers: RefCell<HashMap<SubscriberId, Rc<dyn Fn()>>>,
    subscribers: RefCell<HashMap<SubscriberId, Rc<RefCell<dyn FnMut()>>>>,
}

impl<T: Clone + 'static> Store<T> {
    /// Create a new store with initial state
    pub fn new(initial_state: T) -> Rc<Self> {
        Rc::new(Self {
            state: RefCell::new(initial_state),
            subscribers: RefCell::new(HashMap::new()),
        })
    }

    /// Get a clone of the current state
    pub fn get_state(&self) -> T {
        self.state.borrow().clone()
    }

    /// Subscribe to state changes and return a function to unsubscribe
    pub fn subscribe(&self, callback: impl FnMut() + 'static) -> impl FnOnce() {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let callback = Rc::new(RefCell::new(callback));

        self.subscribers.borrow_mut().insert(id, callback);

        let subscribers_ref = Rc::downgrade(&Rc::new(self.subscribers.clone()));
        move || {
            if let Some(subscribers) = subscribers_ref.upgrade() {
                subscribers.borrow_mut().remove(&id);
            }
        }
    }

    /// Update the state using a mutator function and notify subscribers
    pub fn set<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        // Update state
        updater(&mut self.state.borrow_mut());

        // Notify subscribers
        for sub in self.subscribers.borrow().values() {
            let mut cb = sub.borrow_mut();
            (cb)();
        }
    }

    /// Create a hook to access the store in Dioxus components
    // pub fn use_store<S, Selector>(store: &Rc<Self>, selector: impl Fn(&T) -> S + 'static) -> S
    // where
    //     S: Clone + PartialEq + 'static,
    // {
    pub fn use_store<S>(store: &Rc<Self>, selector: impl Fn(&T) -> S + 'static) -> S
    where
        S: Clone + PartialEq + 'static,
    {
        // Create the signal
        let mut state = use_signal(|| selector(&store.get_state()));
        let store_clone = store.clone();

        // Subscribe to store changes

        // Store reference needed for both hooks
        // let store = store.clone();

        // Create subscription and store the unsubscribe function
        let unsubscribe = store.subscribe(move || {
            let new_value = selector(&store_clone.get_state());
            // Now we can call state.set() without error
            if state() != new_value {
                state.set(new_value);
            }
        });

        use_drop(move || {
            unsubscribe();
        });

        state()
    }
}

// Store creator - similar to Zustand's create function
pub fn create_store<T, StoreFn>(store_creator: StoreFn) -> Rc<Store<T>>
where
    T: Clone + 'static,
    StoreFn: FnOnce(&mut T) -> (),
{
    // Create default value using Default trait if available
    let mut initial_state = std::mem::MaybeUninit::uninit();
    let initial_state = unsafe {
        store_creator(initial_state.assume_init_mut());
        initial_state.assume_init()
    };

    Store::new(initial_state)
}

// Example usage for AuthStore
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub user: Option<User>,
    pub login_status: StateSlice<bool>,
    pub logout_status: StateSlice<bool>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            login_status: StateSlice::new(),
            logout_status: StateSlice::new(),
        }
    }
}

// pub type AuthStore = Rc<Store<AuthState>>;
#[derive(Clone)]
pub struct AuthStore(Rc<Store<AuthState>>);

thread_local! {
    static AUTH_STORE: std::cell::OnceCell<AuthStore>  = std::cell::OnceCell::new();
}

pub fn use_auth_store() -> AuthStore {
    AUTH_STORE.with(|store| {
        store
            .get_or_init(|| AuthStore(Store::new(AuthState::default())))
            .clone()
    })
}

// #[derive(Clone)]
// pub struct AuthActions {
//     store: AuthStore,
// }

impl AuthStore {
    pub fn inner(&self) -> Rc<Store<AuthState>> {
        self.0.clone()
    }

    pub async fn login(&self, email: String, password: String) {
        tracing::info!("Auth Actions: Logging in");
        self.0.set(|state| {
            state.login_status.status = StateStatus::Loading;
            state.login_status.message = None;
        });

        _ = reqwest::get("http://localhost:3000/").await;

        // Simulate a successful login
        if email == "user@example.com" && password == "password" {
            self.0.set(|state| {
                state.login_status.status = StateStatus::Success;
                state.user = Some(User {
                    id: 1,
                    name: "John Doe".to_string(),
                });
            });
        } else {
            self.0.set(|state| {
                state.login_status.status = StateStatus::Failed;
                state.login_status.message = Some("Invalid email or password".to_string());
            });
        }
        // let check = self.0.get_state().clone();
        let checkx = use_auth_store().inner().get_state();
        tracing::info!("AUTH ACTIONS login finish, {checkx:?}");
    }

    pub async fn logout(&self) {
        tracing::info!("Logout actions");
        self.0.set(|state| {
            state.logout_status.status = StateStatus::Loading;
        });

        tracing::info!("Logout actions-xx");

        self.0.set(|state| {
            state.logout_status.status = StateStatus::Success;
            state.user = None;
        });
    }
}

// // Auth store actions
// pub fn use_auth_actions(store: &AuthStore) -> AuthActions {
//     let store = store.clone();
//     AuthActions { store }
// }

// Posts Store
#[derive(Clone)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Clone)]
pub struct PostState {
    pub posts: Vec<Post>,

    pub list_status: StateSlice<bool>,
    pub add_status: StateSlice<Option<Post>>,
    pub edit_status: StateSlice<Option<Post>>,
    pub delete_status: StateSlice<bool>,
}

impl Default for PostState {
    fn default() -> Self {
        Self {
            posts: Vec::<Post>::new(),
            list_status: StateSlice::new(),
            add_status: StateSlice::new(),
            edit_status: StateSlice::new(),
            delete_status: StateSlice::new(),
        }
    }
}

pub type PostsStore = Rc<Store<PostState>>;

// Create the posts store
pub fn create_posts_store() -> PostsStore {
    Store::new(PostState::default())
}

pub fn use_posts_actions(store: &PostsStore) -> impl Clone {
    let store = store.clone();

    #[derive(Clone)]
    struct PostsActions {
        store: PostsStore,
    }

    impl PostsActions {
        pub async fn list_posts(&self) {
            self.store.set(|state| {
                state.list_status.status = StateStatus::Loading;
            });

            // Simulate an asynchronous fetch request
            task::sleep(Duration::from_secs(2)).await;

            // Simulate a successful fetch
            self.store.set(|state| {
                state.posts = vec![
                    Post {
                        id: 1,
                        title: "Post 1".to_string(),
                        content: "Content 1".to_string(),
                    },
                    Post {
                        id: 2,
                        title: "Post 2".to_string(),
                        content: "Content 2".to_string(),
                    },
                ];
                state.list_status.status = StateStatus::Success;
            });
        }

        pub async fn add_post(&self, title: String, content: String) {
            self.store.set(|state| {
                state.add_status.status = StateStatus::Loading;
            });

            task::sleep(Duration::from_secs(2)).await;

            self.store.set(|state| {
                let id = state.posts.len() as u32 + 1;
                let post = Post { id, title, content };
                state.posts.push(post.clone());
                state.add_status.status = StateStatus::Success;
            });
        }

        pub async fn delete_post(&self, id: u32) {
            self.store.set(|state| {
                state.delete_status.status = StateStatus::Loading;
            });

            task::sleep(Duration::from_secs(2)).await;

            self.store.set(|state| {
                state.posts.retain(|post| post.id != id);
                state.delete_status.status = StateStatus::Success;
            });
        }

        pub async fn edit_post(&self, id: u32, title: String, content: String) {
            self.store.set(|state| {
                state.edit_status.status = StateStatus::Loading;
            });

            task::sleep(Duration::from_secs(2)).await;

            self.store.set(|state| {
                if let Some(post) = state.posts.iter_mut().find(|post| post.id == id) {
                    post.title = title;
                    post.content = content;
                }
                state.edit_status.status = StateStatus::Success;
            });
        }
    }

    PostsActions { store }
}
