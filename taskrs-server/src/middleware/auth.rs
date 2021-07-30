use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use actix_web::http::{HeaderName, HeaderValue, Method};
use actix_web::web::Data;
use futures::future::{ok, Ready};
use futures::Future;

use crate::db::DbPool;

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

#[allow(clippy::type_complexity)]
impl<S, B> Service for AuthenticationMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        debug!("Authentication Middleware");
        let mut authenticate_pass: bool = false;

        // Bypass some account routes
        let headers = req.headers_mut();
        headers.append(HeaderName::from_static("content-length"), HeaderValue::from_static("true"));
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            // Get Database Pool
            if let Some(_pool) = req.app_data::<Data<DbPool>>() {
                // Get Authorization Header
                match req.headers().get("authorization") {
                    None => {
                        debug!("Authorization Header was not set");
                    }
                    Some(auth_header) => {
                        if let Ok(auth_str) = auth_header.to_str() {
                            // Check if Bearer Token
                            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                                // Trim Bearer word
                                let token: &str = auth_str[6..auth_str.len()].trim();
                                // Decode token
                                match crate::utils::decode_token(token) {
                                    Ok(_data) => {
                                        authenticate_pass = true;
                                    }
                                    Err(err) => {
                                        error!("Error while decoding token: {}", err);
                                    }
                                }
                            } else {
                                debug!("Header value is no bearer token");
                            }
                        }
                    }
                }
            } else {
                // No Pool found
                unreachable!();
            }
        }

        if authenticate_pass {
            debug!("Authentication succeeded");
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            debug!("Authentication failed");
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .finish()
                        .into_body(),
                ))
            })
        }
    }
}

