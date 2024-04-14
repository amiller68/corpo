#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use corpo::app::*;
    use corpo::config::Config;
    use corpo::ipfsserv::ipfs_and_error_handler;
    use corpo::state::AppState;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // Get the configuration from the environment
    let conf = Config::from_env().unwrap();
    let state = AppState::from_config(config).await.unwrap();

    let leptos_options = state.leptos_options.clone();

    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(ipfs_and_error_handler)
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
