use actix_web::{web, HttpRequest, HttpResponse};
use reqwest::{Client, Method};

pub async fn proxy(
    req: HttpRequest,
    body: web::Bytes,
    target: String,
    prefix: &str,
) -> HttpResponse {
    let client = Client::new();

    let unsplit_uri = req.uri().to_string();
    let uri = unsplit_uri.strip_prefix(prefix).unwrap_or_default();
    let forward_url = format!("{}{}", target, uri);
    // println!("forward url: {}", forward_url);

    let req_method: Method = req.method().to_string().parse().unwrap();
    let mut forward_req = client.request(req_method, forward_url).body(body.clone());

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
        Err(e) => HttpResponse::InternalServerError().body(format!("Proxy error: P{}", e)),
    }
}
