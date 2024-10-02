use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod error;
mod pages;

use pages::{AboutPage, BlogPage, BlogPost, ErrorPage, HomePage};

pub use error::WebAppError;

#[component]
pub fn WebApp() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (menu_open, set_menu_open) = create_signal(false);
    let menu_dialog_ref: NodeRef<html::Dialog> = create_node_ref::<html::Dialog>();

    let toggle_menu_dialog = move |_| {
        // Gauranteed to be initialized
        let menu_dialog = menu_dialog_ref.get().unwrap();
        let menu_dialog_open = menu_dialog.open();

        // Check if the click happened outside the menu
        menu_dialog.set_open(!menu_dialog_open);
        set_menu_open(!menu_dialog_open);

        // Toggle the body-no-scroll class
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();
        if !menu_dialog_open {
            body.class_list().add_1("body-no-scroll").unwrap();
        } else {
            body.class_list().remove_1("body-no-scroll").unwrap();
        }
    };

    view! {

      <Link rel="icon" sizes="32x32" href="/favicon.ico"/>
      <Link rel="preconnect" href="https://fonts.googleapis.com"/>
      <Link rel="preconnect" href="https://fonts.gstatic.com"/>
      <Link href="https://fonts.googleapis.com/css2?family=Roboto+Mono:ital,wght@0,100..700;1,100..700&family=VT323&display=swap" rel="stylesheet"/>
      <Script src="https://unpkg.com/htmx.org@2.0.0-alpha1/dist/htmx.min.js"/>

      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/assets/corpo.css"/>

      // sets the document title
      <Title text="Krondor"/>

      // content for this welcome page
      <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(WebAppError::NotFound);
            view! {
                <ErrorPage outside_errors/>
            }
            .into_view()
        }>
          <header class="relative">
          <dialog
              on:click=toggle_menu_dialog
              ref=menu_dialog_ref>
              <div class="menu">
                  <nav>
                      <ul>
                          <li><A href="">Home</A></li>
                          <li><A href="about">About</A></li>
                          <li><A href="blog">Blog</A></li>
                      </ul>
                  </nav>
                  <span
                    id="menu-close">
                    <p>
                    > X
                    </p>
                  </span>
              </div>
          </dialog>
              <div class="container mx-auto flex justify-between items-center h-[4rem]">
                  <span id="banner">
                    <A href="">Krondor</A>
                  </span>
                  <div class="flex items-center">
                      <input type="checkbox" id="menu-toggle" class="hidden m2-2 form-checkbox" on:click=toggle_menu_dialog />
                      <label for="menu-toggle" class="text-sm cursor-pointer">
                          <span id="menu-icon" class="inline-block w-7 h-7 margin-1 border-2 border-black rounded"
                              style:background-color=move || if menu_open.get() { "transparent" } else { "black" }></span>
                      </label>
                  </div>
              </div>
          </header>
          <main>
              <Routes>
                <Route path="" view=HomePage/>
                <Route path="about" view=AboutPage/>
                <Route path="blog" view=BlogPage/>

                    <Route path="blog/:name" view=BlogPost/>
              </Routes>
          </main>
      </Router>
    }
}
