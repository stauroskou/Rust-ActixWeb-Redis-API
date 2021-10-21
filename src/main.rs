use actix_web::{App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use std::*;
mod config;
mod services;
extern crate redis;
mod DAO;
mod cache;
use tokio::runtime::Runtime;
use tokio::task;
use tracing::{error, Level};
use tracing_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = config::Config::get_config();
    match conf {
        Ok(conf) => {
            let timer: u64 = conf.cache_timeout;
            let bind: String = conf.server_bind;
            let sentry_key: String = conf.sentry_key;
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .init();

            let runtime = Runtime::new().unwrap();
            runtime.block_on(async move {
                task::spawn_blocking(move || {
                    tokio::spawn(async move {
                        cache::Configuration::init(timer).await;
                    })
                });
            });
            let _guard = sentry::init((
                sentry_key,
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            ));
            let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);
            HttpServer::new(move || {
                App::new()
                    .wrap(prometheus.clone())
                    .wrap(sentry_actix::Sentry::new())
                    .configure(services::httpconfig::config)
            })
            .bind(bind)?
            .run()
            .await
        }
        Err(err) => {
            sentry::capture_error(&err);
            error!("{}", err);
            Err(err)
        }
    }
}
