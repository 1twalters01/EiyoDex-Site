use actix_web::{
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    web,
};
use reqwest::{
    Client,
    Method,
};
use core::panic;
use std::env;

// async fn proxy(req: HttpRequest, body: web::Bytes, target: String) -> HttpResponse {
async fn proxy(req: HttpRequest, body: web::Bytes, target: String, prefix: &str) -> HttpResponse {
    let client = Client::new();

    let unsplit_uri = req.uri().to_string();
    let uri = unsplit_uri.strip_prefix(prefix).unwrap_or_default();
    let forward_url = format!("{}{}", target, uri);
    println!("forward url: {}", forward_url);

    let req_method: Method = req.method().to_string().parse().unwrap();
    let mut forward_req = client
        .request(req_method, forward_url)
        .body(body.clone());

    for (name, value) in req.headers().iter() {
        if let Ok(val_str) = value.to_str() {
            forward_req = forward_req.header(name.as_str(), val_str);
        }
    }

    match forward_req.send().await {
        Ok(resp) => {
            let status = actix_web::http::StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
            let headers = resp.headers().clone();
            let bytes = resp.bytes().await.unwrap_or_default();

            let mut client_resp = HttpResponse::build(status);
            for (name, value) in headers.iter() {
                if let Ok(val_str) = value.to_str() {
                    client_resp.append_header((name.as_str(), val_str));
                }
            }
            client_resp.body(bytes)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Proxy error: P{}", e)),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let workspace_dir = match current_dir.parent(){
        Some(path) => path,
        None => panic!("invalid directory"),
    };
    let workspace_env_str = workspace_dir.to_path_buf().into_os_string()
     .into_string().unwrap() + "/.env";
    let workspace_env = std::path::Path::new(
        &workspace_env_str
        );
    let workspace_env = std::path::Path::new(".env");
    println!("workspace env: {:?}", workspace_env);
    dotenvy::from_path(workspace_env).expect("Failed to load .env file");
    let proxy_address = env::var("PROXY_SERVER_URL").expect("PROXY_SERVER_URL missing");
    println!("Proxy at: {:?}", proxy_address);

    HttpServer::new(move || {
        App::new()
            .route("/docs/{tail:.*}", web::to({
                let docs_server = format!(
                    "http://{}:{}",
                    env::var("DOCS_SERVER_HOST").unwrap(),
                    env::var("DOCS_SERVER_PORT").unwrap()
                    );
                move |req, body| proxy(req, body, docs_server.clone(), "/docs")
            }))
            .route("/research/{tail:.*}", web::to({
                let research_server = format!(
                    "http://{}:{}",
                    env::var("RESEARCH_SERVER_HOST").unwrap(),
                    env::var("RESEARCH_SERVER_PORT").unwrap()
                    );
                move |req, body| proxy(req, body, research_server.clone(), "/research")
            }))
            .route("/forum/{tail:.*}", web::to({
                let forum_server = format!(
                    "http://{}:{}",
                    env::var("FORUM_SERVER_HOST").unwrap(),
                    env::var("FORUM_SERVER_PORT").unwrap()
                    );
                move |req, body| proxy(req, body, forum_server.clone(), "/forum")
            }))
            .route("/{tail:.*}", web::to({
                let home_server = format!(
                    "http://{}:{}",
                    env::var("HOME_SERVER_HOST").unwrap(),
                    env::var("HOME_SERVER_PORT").unwrap()
                    );
                move |req, body| proxy(req, body, home_server.clone(), "")
            }))
    })
    .bind(proxy_address)?
    .run()
    .await
}
