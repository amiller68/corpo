use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (menu_open, set_menu_open) = create_signal(false);
    let menu_dialog_ref: NodeRef<html::Dialog> = create_node_ref::<html::Dialog>();

    let toggle_menu_dialog = move |_| {
        // Gauranteed to be initialized
        let menu_dialog = menu_dialog_ref.get().unwrap();
        let menu_dialog_open = menu_dialog.open();
        menu_dialog.set_open(!menu_dialog_open);
        set_menu_open(!menu_dialog_open);
    };

    view! {

      <Link rel="preconnect" href="https://fonts.googleapis.com"/>
      <Link rel="preconnect" href="https://fonts.gstatic.com"/>
      <Link href="https://fonts.googleapis.com/css2?family=Roboto+Mono:ital,wght@0,100..700;1,100..700&family=VT323&display=swap" rel="stylesheet"/>


      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/corpo.css"/>

      // sets the document title
      <Title text="Krondor"/>

      // content for this welcome page
      <Router fallback=|| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(AppError::NotFound);
          view! {
              <ErrorTemplate outside_errors/>
          }
          .into_view()
      }>
          <dialog
              id="menu-dialog"
              ref=menu_dialog_ref>
              <div id="menu">
                  <nav>
                      <ul on:click=toggle_menu_dialog>
                          <li><A href="">Home</A></li>
                          <li><A href="about">About</A></li>
                      </ul>
                  </nav>
                  <button on:click=toggle_menu_dialog>
                      <span> > X </span>
                  </button>
              </div>
          </dialog>
          <header class="relative">
              <div class="container mx-auto flex justify-between items-center h-[4rem]">
                  <span class="text-6xl">
                      K r o n d o r
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
              </Routes>
          </main>
      </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
    }
}
