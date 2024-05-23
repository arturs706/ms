use axum_macros::debug_handler;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    http::HeaderMap,
};
use hyper::{Body, Request, Response};


#[debug_handler]
pub async fn fetchusershandler(headers: HeaderMap ) -> impl IntoResponse {
    let client = hyper::Client::new();
    let forwarded_req = Request::builder()
        .method("GET")
        .uri("http://localhost:11000/api/v1/users")
        .header("Authorization", headers.get("Authorization").unwrap())
        .body(Body::empty())
        .unwrap();

    let resp: Response<Body> = client.request(forwarded_req).await.unwrap();
    let body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body));


    match response {
        Ok(response) => response,
        Err(_) => {
            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
            response
        }
    }
}
