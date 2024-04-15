#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::{
        body::Body as AxumBody, extract::State, http::Request, response::IntoResponse, routing::get,
    };
    use corpo::app::*;
    use corpo::config::Config;
    use corpo::file_serve::file_and_error_handler;
    use corpo::state::AppState;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{EnvFilter, Layer};

    async fn leptos_routes_handler(
        State(app_state): State<AppState>,
        axum::extract::State(option): axum::extract::State<leptos::LeptosOptions>,
        request: Request<AxumBody>,
    ) -> axum::response::Response {
        let handler = leptos_axum::render_app_async_with_context(
            option.clone(),
            move || {
                provide_context(app_state.clone());
            },
            move || view! {  <App/> },
        );

        handler(request).await.into_response()
    }

    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let env_filter = EnvFilter::builder().from_env_lossy();

    let stderr_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_writer(non_blocking_writer)
        .with_filter(env_filter);

    tracing_subscriber::registry().with(stderr_layer).init();

    // Get the configuration from the environment
    let config = Config::parse_env().unwrap();
    let state = AppState::from_config(&config).await.unwrap();

    let leptos_options = state.leptos_options.clone();

    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        // .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
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
