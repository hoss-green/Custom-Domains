use axum::{
    extract::Request,
    http::header::HOST,
    response::IntoResponse,
    routing::get,
    Router, ServiceExt,
};
use axum::http::HeaderValue;
use tower::Layer;

#[tokio::main]
async fn main() {
    let middleware = tower::util::MapRequestLayer::new(rewrite_request_uri);
    let router = Router::new().route("/", get(handler));
    let app = middleware.layer(router).into_make_service();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap());
    axum::serve(listener, app).await.unwrap()
}

async fn handler(request: Request) -> impl IntoResponse {
    format!("{:#?}", request).into_response()
}

fn rewrite_request_uri<B>(request: Request<B>) -> Request<B> {
    let mut ret_request = request;
    let headers = ret_request.headers_mut();
    let host = match headers.get(HOST) {
        Some(host) => host.to_str().unwrap(),
        None => "no_host",
    };

    println!("{}", &host);
    headers.remove(HOST);
    let hv = match HeaderValue::from_str("cheeky_breeky_um_ve_dankye.com") {
        Ok(header_val) => header_val,
        Err(_) => {
            panic!("could not create header");
        }
    };

    headers.insert(HOST, hv);

    ret_request
}
// pub async fn run_middleware<B>(
//     headers: HeaderMap,
//     mut request: Request,
//     next: Next,
// ) -> Result<impl IntoResponse, StatusCode> {
//     Ok("Hello middleware".into_response())
// }
