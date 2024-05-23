use axum::{body::BoxBody, http::header::CONTENT_TYPE, response::IntoResponse};
use futures::{future::BoxFuture, ready};
use hyper::{Body, Request, Response};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::Service;



pub struct MergServ<A, B> {
    rest: A,
    rest_ready: bool,
    grpc: B,
    grpc_ready: bool,
}

impl<A, B> MergServ<A, B> {
    pub fn new(rest: A, grpc: B) -> Self {
        Self {
            rest,
            rest_ready: false,
            grpc,
            grpc_ready: false,
        }
    }
}

impl<A, B> Clone for MergServ<A, B>
where
    A: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            rest: self.rest.clone(),
            grpc: self.grpc.clone(),
            // the cloned services probably wont be ready
            rest_ready: false,
            grpc_ready: false,
        }
    }
}

impl<A, B> Service<Request<Body>> for MergServ<A, B>
where
    A: Service<Request<Body>, Error = Infallible>,
    A::Response: IntoResponse,
    A::Future: Send + 'static,
    B: Service<Request<Body>>,
    B::Response: IntoResponse,
    B::Future: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = B::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        loop {
            match (self.rest_ready, self.grpc_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(self.rest.poll_ready(cx)).map_err(|err| match err {})?;
                    self.rest_ready = true;
                }
                (_, false) => {
                    ready!(self.grpc.poll_ready(cx))?;
                    self.grpc_ready = true;
                }
            }
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        assert!(
            self.grpc_ready,
            "grpc service not ready. Did you forget to call `poll_ready`?"
        );
        assert!(
            self.rest_ready,
            "rest service not ready. Did you forget to call `poll_ready`?"
        );
        if is_grpc_request(&req) {
            self.grpc_ready = false;
            
            let future = self.grpc.call(req);
            Box::pin(async move {
                let res = future.await?;
                Ok(res.into_response())
            })
        } else {
            self.rest_ready = false;
            let future = self.rest.call(req);
            Box::pin(async move {
                let res = future.await.map_err(|err| match err {})?;
                Ok(res.into_response())
            })
        }
    }
}

fn is_grpc_request<B>(req: &Request<B>) -> bool {
    req.headers()
       
        .get(CONTENT_TYPE)
        
        .map(|content_type| content_type.as_bytes())
        
        .filter(|content_type| content_type.starts_with(b"application/grpc"))
        .is_some()
        
}
