use anyhow::{Context, Result};
use minijinja::{context, Environment};
use slog::*;
use std::sync::Arc;
use warp::Filter;

mod config;
mod filters;
mod util;

use config::Config;

const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");
const INDEX_HTML_TEMPLATE: &str = include_str!("../site/index.html.template");

pub struct App {
    pub log: Logger,
    pub config: Config,
    pub index_html: String,
}

fn create_logger() -> Logger {
    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    Logger::root(slog_term::FullFormat::new(plain).build().fuse(), o!())
}

fn create_index_page(cfg: &Config) -> String {
    let mut env = Environment::new();
    env.add_template("index.html", INDEX_HTML_TEMPLATE).unwrap();
    let template = env.get_template("index.html").unwrap();
    template
        .render(context! {
            index => cfg.index,
            links => cfg.urls,
        })
        .unwrap()
}

fn main() -> Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    let config = Config::from_file(&config_path)
        .with_context(|| format!("failed to parse \"{}\"", &config_path))?;
    let nthreads = config.server.threads.unwrap_or(2);
    let app = Arc::new(App {
        log: create_logger(),
        index_html: create_index_page(&config),
        config,
    });
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(nthreads)
        .thread_name(format!("{}-worker", PROGRAM_NAME))
        .enable_io()
        .build()
        .expect("failed to build tokio runtime");

    let log_app = app.clone();
    let log = warp::log::custom(move |info| {
        info!(
            log_app.log,
            "{} {} {:?} {} {} ({:?})",
            util::OptFmt(info.remote_addr().map(|s| s.ip())),
            info.method(),
            info.path(),
            info.status().as_u16(),
            util::OptFmt(info.user_agent()),
            info.elapsed(),
        );
    });

    let url_shortener = filters::url_shortener(app.clone());
    rt.block_on(async {
        println!(
            "listening on http://{}:{}",
            app.config.server.host, app.config.server.port
        );
        warp::serve(url_shortener.with(log))
            .run((app.config.server.host, app.config.server.port))
            .await
    });

    Ok(())
}
