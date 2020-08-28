/// Adapted from: https://github.com/actix/examples/blob/master/middleware/src/redirect.rs
/// 8/27/2020

use std::task::{Context, Poll};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, Ready};

pub struct CheckLogin;

impl<S, B> Transform<S> for CheckLogin
	where
		S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type InitError = ();
	type Transform = CheckLoginMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ok(CheckLoginMiddleware { service })
	}
}
pub struct CheckLoginMiddleware<S> {
	service: S,
}

impl<S, B> Service for CheckLoginMiddleware<S>
	where
		S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

	fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}


	/// Validate the bearer token
	/// curl -H "authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiMSJ9.nNpbmWCiz6-exAOkLdl3nQrzh5p-QEhZ3ko18T8vvII" http://127.0.0.1:8080/secret -v	fn call(&mut self, req: ServiceRequest) -> Self::Future {
	fn call(&mut self, req: ServiceRequest) -> Self::Future {
		// We only need to hook into the `start` for this middleware.
		log::debug!("[CheckLoginMiddleware call]");

		if req.path() == "/auth"{
			log::debug!("[CheckLoginMiddleware call] path is /auth");
			Either::Left(self.service.call(req))

		}else{

			log::debug!("[CheckLoginMiddleware call] path is not /auth");

			// (1) Check if theres an authorization token
			// (2) Validate the token
			// (3) Logged in, redirect to content. Otherwise redirect to auth

			let mut is_logged_in; // Change this to see the change in outcome in the browser

			let http_header = req.headers().get(actix_web::http::header::AUTHORIZATION);

			log::debug!("[CheckLoginMiddleware.call] http_header: {:?}", &http_header);

			match http_header {
				None => {
					// deny; redirect to /auth
					is_logged_in = false;
				},
				Some(bearer_jwt_candidate) => {

					// If it has a "Bearer " at the head of the string, good, continue, otherwise, wrong header.
					// remove the "Bearer " from the header
					log::debug!("[CheckLoginMiddleware.call] raw bearer_jwt_candidate: {:?}", &bearer_jwt_candidate);

					if bearer_jwt_candidate.to_str().unwrap().contains("Bearer ") == false {
						is_logged_in = false;
					} else {
						let token_candidate:String = bearer_jwt_candidate.to_str().unwrap().replace("Bearer ", "");
						// the result will either be a valid token remaining in the string or it'll be blank
						// I'm unclear what replace() returns if not found. Checking it was there just in case.

						log::debug!("[CheckLoginMiddleware.call] token_candidate: {:?}", &token_candidate);

						// JWT decode
						let mut validation = jsonwebtoken::Validation::default();
						validation.validate_exp = false;
						let jwt_decode_result = jsonwebtoken::decode::<crate::Claims>(
							&token_candidate,
							&jsonwebtoken::DecodingKey::from_secret(crate::TOKEN_SECRET.as_ref()),
							&validation
						);

						if let Ok(token_data) = jwt_decode_result {
							log::debug!("[CheckLoginMiddleware.call] claim: {:?}", &token_data.claims);
							// ******* ALLOW access (based on user ID if applicable) ******
							// TODO: use other JWT token capabilities to map to authorities
							log::debug!("[CheckLoginMiddleware.call] incoming JWT was validated (user ID: {:?})", &token_data.claims.user_id);
							is_logged_in = true;
						} else {
							log::debug!("[CheckLoginMiddleware.call] jwt decode failed");
							is_logged_in = false;
						}
					}
				}
			}

			log::debug!("[CheckLoginMiddleware.call] is_logged_in: {}", is_logged_in);

			if is_logged_in {
				// pass the request through to wherever they wanted to go
				Either::Left(self.service.call(req))
			} else {
				if req.path() == "/auth" {
					// If they're headed to the login page, all them to pass
					Either::Left(self.service.call(req))
				} else {
					// Everyone else is denied access and is redirected to the login page.
					Either::Right(ok(
						req.into_response(HttpResponse::Unauthorized() .finish().into_body())
					))
					// Either::Right(ok(req.into_response(
					// 	HttpResponse::Found()
					// 		.header(http::header::LOCATION, "/auth")
					// 		.finish()
					// 		.into_body(),
					// )))
				}
			}
		}
	}
}
