use std::sync::Arc;

mod config;
mod filters;

use config::Config;

pub struct App {
    pub log: String,    // TODO Logger
    pub config: Config,
}


#[tokio::main(worker_threads = 2)]
async fn main() -> std::io::Result<()> {
    let config = Config::from_file("config.toml")?;
    let app = Arc::new(App {
        log: "log".to_string(),
        config,
    });

    let url_shortener = filters::url_shortener(app);
    warp::serve(url_shortener).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}
