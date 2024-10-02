use leptos::*;
use reqwest::Client;
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    name: String,
    title: String,
    description: String,
    created_at: OffsetDateTime,
}

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = create_resource(|| (), |_| async move {
        let client = Client::new();
        let url = format!("{}{}", window().location().origin().unwrap(), "/api/v0/blog");
        
        match client.get(&url).send().await {
            Ok(response) => {
                match response.json::<Vec<Post>>().await {
                    Ok(mut posts) => {
                        posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        Ok(posts)
                    },
                    Err(e) => Err(format!("Failed to parse JSON: {}", e)),
                }
            },
            Err(e) => Err(format!("Failed to send request: {}", e)),
        }
    });

    view! {
        <div class="flex flex-col items-center justify-center">
            <h1
                class="relative w-[max-content] font-mono text-4xl font-bold before:absolute before:inset-0 before:animate-typewriter before:bg-white after:absolute after:inset-0 after:w-[0.125em] after:animate-caret after:bg-black">
                "i wrote stuff, take a look"
            </h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match posts.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(Ok(posts)) => view! {
                        <ul class="mt-8 space-y-4">
                            {posts.into_iter().map(|post| view! {
                                <li class="border p-4 rounded-lg">
                                    <h2 class="text-xl font-bold">{post.title}</h2>
                                    <p class="text-gray-600">{post.description}</p>
                                    <p class="text-sm text-gray-400">
                                        post.created_at
                                    </p>
                                </li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_view(),
                    Some(Err(_e)) => view! { <p>"Error: {e}"</p> }.into_view(),
                }}
            </Suspense>
        </div>
    }
}