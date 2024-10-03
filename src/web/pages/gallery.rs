use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageMetadata {
    name: String,
}

#[component]
pub fn GalleryPage() -> impl IntoView {
    let (images, set_images) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    create_effect(move |_| {
        spawn_local(async move {
            let url = format!("{}/api/v0/gallery", window().location().origin().unwrap());
            match reqwest::get(&url).await {
                Ok(response) => match response.json::<Vec<ImageMetadata>>().await {
                    Ok(mut fetched_images) => {
                        fetched_images.sort_by(|a, b| a.name.cmp(&b.name));
                        set_images.set(fetched_images);
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
                <div class="max-w-6xl mx-auto px-4 py-8">
                    <h1
                        class="relative font-mono text-4xl mb-12 font-bold before:absolute before:inset-0 before:animate-typewriter before:bg-white after:absolute after:inset-0 after:w-[0.125em] after:animate-caret after:bg-black">
                        "> doodles and pics"
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
                                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                                    {images.get().into_iter().map(|image| view! {
                                        <A href=format!("/gallery/{}", image.name)
                                           class="block bg-white rounded-lg overflow-hidden shadow-md hover:shadow-xl
                                                  transition-all duration-300 ease-in-out transform hover:scale-105">
                                            <img src=format!("/api/v0/gallery/{}", image.name)
                                                 alt=image.name
                                                 class="w-full h-48 object-cover"/>
                                        </A>
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_view()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn GalleryImage() -> impl IntoView {
    let params = use_params_map();
    let image_name = move || params.with(|params| params.get("name").cloned().unwrap_or_default());

    let (image_loaded, set_image_loaded) = create_signal(false);
    let (not_found, set_not_found) = create_signal(false);

    let image_url = move || format!("/api/v0/gallery/{}", image_name());

    create_effect(move |_| {
        set_image_loaded(false);
        set_not_found(false);
    });

    let on_load = move |_| set_image_loaded(true);
    let on_error = move |_| set_not_found(true);

    view! {
        <div class="min-h-screen flex flex-col">
            <div class="flex-grow overflow-y-auto">
                <div class="max-w-4xl mx-auto px-4 py-8">
                    <h1 class="text-2xl font-bold mb-4">{image_name}</h1>
                    {move || {
                        if not_found.get() {
                            view! { <p class="text-center text-lg text-red-500">"Image not found"</p> }.into_view()
                        } else {
                            view! {
                                <div class="bg-white rounded-lg overflow-hidden shadow-lg">
                                    {move || if !image_loaded.get() {
                                        view! {
                                            <div class="flex justify-center items-center h-64">
                                                <div class="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-gray-900"></div>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }}
                                    <img src=image_url
                                         alt=image_name
                                         class="w-full object-contain max-h-[70vh]"
                                         on:load=on_load
                                         on:error=on_error
                                         style:display=move || if image_loaded.get() { "block" } else { "none" }
                                    />
                                </div>
                            }.into_view()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
