use std::convert::Infallible;
use std::sync::Arc;
use warp::http::Uri;
use warp::Filter;

mod config;

#[derive(Default)]
struct App {
    _log: String,    // Logger
    _config: String, // Config
}

fn with_app(
    app: Arc<App>,
) -> impl Filter<Extract = (Arc<App>,), Error = Infallible> + Clone {
    warp::any().map(move || app.clone())
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    let app = Arc::new(App {
        ..Default::default()
    });

    let index = warp::get()
        .and(with_app(app.clone()))
        .and(warp::path::end())
        .map(|_| {
            // TODO return json list of routes
            println!("you hit /");
            warp::reply()
        });

    let redirect = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(with_app(app.clone()))
        .map(|p, _| {
            // TODO redirect to appropriate backend
            println!("you hit /{}", p);
            warp::redirect::temporary(Uri::from_static("/"))
        });

    let routes = index.or(redirect);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
