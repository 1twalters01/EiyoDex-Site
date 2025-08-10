use actix_web::{web, App, HttpServer};
use std::env;

pub mod load_env;
pub mod proxy;
use load_env::load_env;
use proxy::proxy;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    load_env();
    let proxy_address = env::var("PROXY_SERVER_URL").expect("PROXY_SERVER_URL missing");
    println!(
        "Proxy at: \x1b]8;;http://{0}\x1b\\http://{0}\x1b]8;;\x1b\\",
        proxy_address
    );
    HttpServer::new(move || {
        App::new()
            .route(
                "/docs/{tail:.*}",
                web::to({
                    let docs_server = format!(
                        "http://{}:{}",
                        env::var("DOCS_SERVER_HOST").unwrap(),
                        env::var("DOCS_SERVER_PORT").unwrap()
                    );
                    move |req, body| proxy(req, body, docs_server.clone(), "/docs")
                }),
            )
            .route(
                "/research/{tail:.*}",
                web::to({
                    let research_server = format!(
                        "http://{}:{}",
                        env::var("RESEARCH_SERVER_HOST").unwrap(),
                        env::var("RESEARCH_SERVER_PORT").unwrap()
                    );
                    move |req, body| proxy(req, body, research_server.clone(), "/research")
                }),
            )
            .route(
                "/forum/{tail:.*}",
                web::to({
                    let forum_server = format!(
                        "http://{}:{}",
                        env::var("FORUM_SERVER_HOST").unwrap(),
                        env::var("FORUM_SERVER_PORT").unwrap()
                    );
                    move |req, body| proxy(req, body, forum_server.clone(), "/forum")
                }),
            )
            .route(
                "/{tail:.*}",
                web::to({
                    let home_server = format!(
                        "http://{}:{}",
                        env::var("HOME_SERVER_HOST").unwrap(),
                        env::var("HOME_SERVER_PORT").unwrap()
                    );
                    move |req, body| proxy(req, body, home_server.clone(), "")
                }),
            )
    })
    .bind(proxy_address)?
    .run()
    .await
}
