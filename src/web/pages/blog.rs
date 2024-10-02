use leptos::*;
use leptos_router::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::format_description;
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    name: String,
    title: String,
    description: String,
    created_at: OffsetDateTime,
}

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = create_resource(
        || (),
        |_| async move {
            let client = Client::new();
            let url = format!(
                "{}{}",
                window().location().origin().unwrap(),
                "/api/v0/blog"
            );
            match client.get(&url).send().await {
                Ok(response) => match response.json::<Vec<Post>>().await {
                    Ok(mut posts) => {
                        posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        Ok(posts)
                    }
                    Err(e) => Err(format!("Failed to parse JSON: {}", e)),
                },
                Err(e) => Err(format!("Failed to send request: {}", e)),
            }
        },
    );

    view! {
        <div class="flex flex-col items-start justify-center w-full">
            <h1
                class="relative font-mono text-4xl font-bold before:absolute before:inset-0 before:animate-typewriter before:bg-white after:absolute after:inset-0 after:w-[0.125em] after:animate-caret after:bg-black">
                "> i wrote stuff"
            </h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match posts.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(Ok(posts)) => view! {
                        <ul class="mt-8 space-y-4 w-full">
                            {posts.into_iter().map(|post| view! {
                                <li class="border p-4 rounded-lg">
                                    <A href=format!("/blog/{}", post.name)>
                                        <h2 class="text-xl font-bold">{post.title}</h2>
                                        <p class="text-gray-600">{post.description}</p>
                                        <p class="text-sm text-gray-400">
                                        {
                                            let time_format = format_description::parse("[year]-[month]-[day]").unwrap();
                                            post.created_at.format(&time_format).unwrap()
                                        }
                                        </p>
                                    </A>
                                </li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_view(),
                    Some(Err(e)) => view! { <p>"Error: {e}"</p> }.into_view(),
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let post_name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());

    let post = create_resource(post_name, |name| async move {
        leptos::logging::log!("Fetching blog post: {}", name);
        // first get the post metadata
        //  from the /api/blog endpoint and finding where the name
        //   matches

        let client = Client::new();
        let metadataUrl = format!("{}/api/v0/blog", window().location().origin().unwrap(),);
        let post = match client.get(&metadataUrl).send().await {
            Ok(response) => match response.json::<Vec<Post>>().await {
                Ok(posts) => posts.into_iter().find(|post| post.name == name),
                Err(e) => {
                    leptos::logging::log!("Failed to parse JSON: {}", e);
                    None
                }
            },
            Err(e) => {
                leptos::logging::log!("Failed to send request: {}", e);
                None
            }
        };
        leptos::logging::log!("Post metadata: {:?}", post);

        let url = format!(
            "{}/api/v0/blog/{}",
            window().location().origin().unwrap(),
            name
        );

        let content = match client.get(&url).send().await {
            Ok(response) => match response.text().await {
                Ok(content) => Some(content),
                Err(e) => {
                    leptos::logging::log!("Failed to parse JSON: {}", e);
                    None
                }
            },
            Err(e) => {
                leptos::logging::log!("Failed to send request: {}", e);
                None
            }
        };

        leptos::logging::log!("Post content: {:?}", content);

        (post, content)
    });

    view! {
        <div class="flex flex-col items-start justify-center w-full">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || match post.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some((Some(post), Some(content))) => view! {
                        <div class="prose mt-8 w-full" inner_html=content/>

                    }.into_view(),
                    _ => view! { <p>"Error: {e}"</p> }.into_view(),
                }}
            </Suspense>
        </div>
    }
}
