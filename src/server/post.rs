use dioxus::prelude::*;

use crate::stores::post::Post;

fn posts() -> Vec<Post> {
    vec![
        Post {
            id: 1,
            title: "Rust vs. Go: Performance Showdown".to_string(),
            content: "Benchmarking shows Rust outperforms Go in CPU-intensive tasks, while Go shines in quick compilation and simpler concurrency models.".to_string(),
        },
        Post {
            id: 2,
            title: "The Rise of WebAssembly".to_string(),
            content: "WebAssembly is revolutionizing web performance by allowing C++, Rust and other languages to run at near-native speed in browsers.".to_string(),
        },
        Post {
            id: 3,
            title: "Elden Ring: A Programmer's Perspective".to_string(),
            content: "FromSoftware's open world design brilliantly uses procedural generation while maintaining handcrafted quality - a lesson for game developers.".to_string(),
        },
        Post {
            id: 4,
            title: "AI Pair Programming: GitHub Copilot Review".to_string(),
            content: "After three months with Copilot, I've found it speeds up boilerplate code writing but still requires careful review for logic and security issues.".to_string(),
        },
        Post {
            id: 5,
            title: "Mechanical Keyboards for Programmers".to_string(),
            content: "Cherry MX Browns offer the best balance between typing comfort and noise level for office environments. Your wrists will thank you.".to_string(),
        },
        Post {
            id: 6,
            title: "Raytracing in Modern Games".to_string(),
            content: "NVIDIA's DLSS 3.0 combined with raytracing finally delivers the holy grail of real-time photorealistic graphics without sacrificing framerate.".to_string(),
        },
        Post {
            id: 7,
            title: "Docker vs. Kubernetes for Small Teams".to_string(),
            content: "Small dev teams should stick with Docker Compose; Kubernetes adds unnecessary complexity until you're managing dozens of microservices.".to_string(),
        },
        Post {
            id: 8,
            title: "The Metaverse: Gaming's Next Frontier?".to_string(),
            content: "Despite the hype, metaverse gaming needs more than VR headsets - it requires solving fundamental issues in digital ownership and interoperability.".to_string(),
        },
        Post {
            id: 9,
            title: "Python's GIL: Understanding the Bottleneck".to_string(),
            content: "Python's Global Interpreter Lock restricts true multithreading, but async programming and multiprocessing offer viable workarounds.".to_string(),
        },
        Post {
            id: 10,
            title: "Indie Game Development with Rust".to_string(),
            content: "The Bevy engine combined with Rust's safety guarantees is making indie game development more accessible without sacrificing performance.".to_string(),
        },
    ]
}

#[server]
pub async fn fetch_all_posts() -> Result<Vec<Post>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(posts())
}

#[server]
pub async fn fetch_post_by_id(id: i32) -> Result<Option<Post>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(posts().into_iter().find(|post| post.id == id))
    // .ok_or(ServerFnError::NotFound)?)
}
