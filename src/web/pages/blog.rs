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
    let (posts, set_posts) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    create_effect(move |_| {
        spawn_local(async move {
            let client = Client::new();
            let url = format!("{}/api/v0/blog", window().location().origin().unwrap());
            match client.get(&url).send().await {
                Ok(response) => match response.json::<Vec<Post>>().await {
                    Ok(mut fetched_posts) => {
                        fetched_posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                        set_posts.set(fetched_posts);
                        set_loading.set(false);
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to parse JSON: {}", e)));
                        set_loading.set(false);
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to send request: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="min-h-screen flex flex-col">
            <div class="flex-grow overflow-y-auto">
                <div class="max-w-3xl mx-auto px-4 py-8">
                    <h1
                        class="relative font-mono text-4xl font-bold mb-8 before:absolute before:inset-0 before:animate-typewriter before:bg-white after:absolute after:inset-0 after:w-[0.125em] after:animate-caret after:bg-black">
                        "> blog stuff"
                    </h1>
                    {move || {
                        if loading.get() {
                            view! {
                                <div class="flex justify-center items-center h-64">
                                    <div class="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-gray-900"></div>
                                </div>
                            }.into_view()
                        } else if let Some(err) = error.get() {
                            view! { <p class="text-center text-red-500">"Error: " {err}</p> }.into_view()
                        } else {
                            view! {
                                <ul class="space-y-6">
                                    {posts.get().into_iter().map(|post| view! {
                                        <li class="bg-white rounded-lg overflow-hidden shadow-md hover:shadow-xl
                                                   transition-all duration-300 ease-in-out transform hover:scale-102">
                                            <A href=format!("/blog/{}", post.name) class="block p-6">
                                                <h2 class="text-2xl font-bold mb-2">{post.title}</h2>
                                                <p class="text-gray-600 mb-2">{post.description}</p>
                                                <p class="text-sm text-gray-500">
                                                {
                                                    let time_format = format_description::parse("[year]-[month]-[day]").unwrap();
                                                    post.created_at.format(&time_format).unwrap()
                                                }
                                                </p>
                                            </A>
                                        </li>
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }.into_view()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let post_name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());

    let (post, set_post) = create_signal(None::<Post>);
    let (content, set_content) = create_signal(String::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    create_effect(move |_| {
        let name = post_name();
        spawn_local(async move {
            let client = Client::new();
            let metadataUrl = format!("{}/api/v0/blog", window().location().origin().unwrap());
            match client.get(&metadataUrl).send().await {
                Ok(response) => match response.json::<Vec<Post>>().await {
                    Ok(posts) => {
                        if let Some(found_post) = posts.into_iter().find(|p| p.name == name) {
                            set_post.set(Some(found_post));

                            // Fetch post content
                            let contentUrl = format!(
                                "{}/api/v0/blog/{}",
                                window().location().origin().unwrap(),
                                name
                            );
                            match client.get(&contentUrl).send().await {
                                Ok(response) => match response.text().await {
                                    Ok(fetched_content) => {
                                        set_content.set(fetched_content);
                                        set_loading.set(false);
                                    }
                                    Err(e) => {
                                        set_error
                                            .set(Some(format!("Failed to fetch content: {}", e)));
                                        set_loading.set(false);
                                    }
                                },
                                Err(e) => {
                                    set_error.set(Some(format!(
                                        "Failed to send content request: {}",
                                        e
                                    )));
                                    set_loading.set(false);
                                }
                            }
                        } else {
                            set_error.set(Some("Post not found".to_string()));
                            set_loading.set(false);
                        }
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to parse JSON: {}", e)));
                        set_loading.set(false);
                    }
                },
                Err(e) => {
                    set_error.set(Some(format!("Failed to send request: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    });

    view! {
        <div class="min-h-screen flex flex-col">
            <div class="flex-grow overflow-y-auto">
                <div class="max-w-3xl mx-auto px-4 py-8">
                    {move || {
                        if loading.get() {
                            view! { <p class="text-center text-lg">"Loading..."</p> }.into_view()
                        } else if let Some(err) = error.get() {
                            view! { <p class="text-center text-red-500">"Error: " {err}</p> }.into_view()
                        } else if let Some(post) = post.get() {
                            view! {
                                <article class="prose lg:prose-xl max-w-none">
                                    <div class="mb-8 p-6 bg-gray-50 border-l-4 border-gray-300 rounded-r-lg shadow-sm">
                                        <h1 class="text-4xl font-bold mb-3">{post.title}</h1>
                                        <p class="text-xl text-gray-600 mb-2">{post.description}</p>
                                        <p class="text-sm text-gray-500">
                                        {
                                            let time_format = format_description::parse("[year]-[month]-[day]").unwrap();
                                            post.created_at.format(&time_format).unwrap()
                                        }
                                        </p>
                                    </div>
                                    <div
                                        class="[&>p]:mb-6 [&>h2]:text-2xl [&>h2]:font-bold [&>h2]:mt-8 [&>h2]:mb-4
                                               [&>h3]:text-xl [&>h3]:font-bold [&>h3]:mt-6 [&>h3]:mb-3
                                               [&>img]:mx-auto [&>img]:my-8
                                               [&>pre]:bg-gray-100 [&>pre]:p-4 [&>pre]:rounded-md [&>pre]:overflow-x-auto
                                               [&>pre]:text-gray-800 [&>pre]:border [&>pre]:border-gray-300
                                               [&>:not(pre)>code]:bg-gray-200 [&>:not(pre)>code]:text-gray-800 
                                               [&>:not(pre)>code]:px-1 [&>:not(pre)>code]:py-0.5 [&>:not(pre)>code]:rounded
                                               [&>:not(pre)>code]:border [&>:not(pre)>code]:border-gray-300"
                                        inner_html=content.get()
                                    />
                                </article>
                            }.into_view()
                        } else {
                            view! { <p class="text-center text-lg">"Post not found"</p> }.into_view()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
