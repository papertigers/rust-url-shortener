use std::convert::Infallible;
use std::sync::Arc;
use warp::http::{StatusCode, Uri};
use warp::Filter;

use crate::App;

enum MappedUri {
    Found(Uri),
    NotFound,
}

impl warp::Reply for MappedUri {
    fn into_response(self) -> warp::reply::Response {
        match self {
            Self::Found(uri) => warp::redirect::temporary(uri).into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

fn with_app(
    app: Arc<App>,
) -> impl Filter<Extract = (Arc<App>,), Error = Infallible> + Clone {
    warp::any().map(move || app.clone())
}

pub fn url_shortener(
    app: Arc<App>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index_json = warp::get()
        .and(warp::path::end())
        .and(with_app(app.clone()))
        .and(warp::header::exact_ignore_case(
            "accept",
            "application/json",
        ))
        .map(|app: Arc<App>| warp::reply::json(&app.config.urls));

    let index = warp::get()
        .and(warp::path::end())
        .and(with_app(app.clone()))
        .map(|app: Arc<App>| warp::reply::html(app.index_html.clone()));

    let redirect = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(with_app(app))
        .map(|path, app: Arc<App>| match app.config.urls.get(&path) {
            Some(uri) => MappedUri::Found((*uri).clone()),
            None => MappedUri::NotFound,
        });

    index_json.or(index).or(redirect)
}
