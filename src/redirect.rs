/// Adapted from: https://github.com/actix/examples/blob/master/middleware/src/redirect.rs
/// 8/27/2020

use std::task::{Context, Poll};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};

/// For future JWT implementation
/*#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims{
	pub user_id:String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Login{
	password:String,
}*/

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
	/// Technically it should be possible to include both BASIC and BEARER authentication, just in a
	/// list in a single AUTHORIZATION header. So might need to parse a comma as well. This will only
	/// parse BASIC for now.
	/// http://greenbytes.de/tech/webdav/rfc7235.html#header.authorization
	fn call(&mut self, req: ServiceRequest) -> Self::Future {
		// We only need to hook into the `start` for this middleware.

		log::debug!("[CheckLoginMiddleware] path: {:?}", req.path());

		// Check for both BASIC and BEARER credentials
		// If there are no BEARER, then see if there are BASIC cred
		// If there are BEARER, login in that way.
		// If there are neither, send WWW-AUTHENTICATE header in 401 response

		let mut is_logged_in = false; // Change this to see the change in outcome in the browser


		// BASIC authorization
		let auth_header = req.headers().get("authorization");

		match auth_header {

			None => {
				// No AUTHORIZATION header so ask client to send it by sending a 401 w/a WWW-AUTHENTICATE header
				log::debug!("[CheckLoginMiddleware] authorization header: None");
				is_logged_in = false;
				// HttpResponse::Unauthorized().header(actix_web::http::header::WWW_AUTHENTICATE, "Basic realm=\"Developer Portal\"").finish()
			},

			Some(auth_header) => {
				// Extract "BASIC username:password" somewhat per RFC; technically there could be a list of auth methods
				log::debug!("[CheckLoginMiddleware] raw authorization header: {:?}", &auth_header.to_str());
				// TODO: make case insensitive, confirm per RFC
				// TODO: Check for other kinds of authentication headers besides BASIC
				let auth_header: String = auth_header.to_str().unwrap().replace("Basic ", "");
				log::debug!("[CheckLoginMiddleware] username/password base64 encoded: {}", auth_header.as_str());
				let auth_header: Vec<u8> = auth_header.into_bytes();
				let auth_header: Vec<u8> = base64::decode(auth_header).unwrap();
				let auth_header: String = String::from_utf8(auth_header.to_owned()).unwrap();
				let credentials: Vec<&str> = auth_header.split(":").collect();
				// let username = credentials[0];
				let password = credentials[1];

				log::debug!("[CheckLoginMiddleware] username: {}", credentials[0]);
				log::debug!("[CheckLoginMiddleware] password: {}", credentials[1]);

				if password == crate::FAKE_USER_PASSWORD {
					log::debug!("[CheckLoginMiddleware] Success. Password matches.");

					// TODO: remove this and also check for existence of a BEARER token, further up, before checking for a BASIC
					is_logged_in = true;
				}



				// TODO: Generate signed JWT upon login -- currently disabled below; requires a client that can save it
				// and add it to an "Authorization: Bearer [token]" http header.
				/*
				let user_claim = Claims{
					user_id: "1".to_owned(),
				};

				log::debug!("[CheckLoginMiddleware] Creating bearer header and done.");

				// See the JWT in the response to the client
				// curl --user name:topsecret 127.0.0.1:8080/auth -v
				/*
					*   Trying 127.0.0.1...
					* TCP_NODELAY set
					* Connected to 127.0.0.1 (127.0.0.1) port 8080 (#0)
					* Server auth using Basic with user 'name'
					> GET /auth HTTP/1.1
					> Host: 127.0.0.1:8080
					> Authorization: Basic bmFtZTp0b3BzZWNyZXQ=
					> User-Agent: curl/7.64.1
					> Accept:
					>
					< HTTP/1.1 200 OK
					< content-length: 0
					< authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiMSJ9.nNpbmWCiz6-exAOkLdl3nQrzh5p-QEhZ3ko18T8vvII
					< date: Fri, 28 Aug 2020 05:45:06 GMT
					<
					* Connection #0 to host 127.0.0.1 left intact
					* Closing connection 0
				 */
				jsonwebtoken::encode(&jsonwebtoken::Header::default(), &user_claim, &jsonwebtoken::EncodingKey::from_secret(TOKEN_SECRET.as_ref()))
					.map(|token|
					{
						HttpResponse::Ok()
							.header(actix_web::http::header::LOCATION, "/")
							.header(actix_web::http::header::AUTHORIZATION, format! ("Bearer {}", token))
							.finish()
					}).unwrap_or(HttpResponse::InternalServerError().into())
			*/
			}
		}




		// BEARER
		/*
		if req.path() == "/auth"{
			log::debug!("[CheckLoginMiddleware] path is /auth");
			Either::Left(self.service.call(req))

		}else{

			log::debug!("[CheckLoginMiddleware] path is not /auth");

			// (1) Check if theres an authorization token
			// (2) Validate the token
			// (3) Logged in, redirect to content. Otherwise redirect to auth


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

		}


		 */

		/*
			States of the login state machine:
				- path_that_doesn't_require_auth(like /auth)
				- logged_in

		 */

		match req.path(){

			"/auth" => {
				Either::Left(self.service.call(req))
			},

			// "/" => {
			// 	//redirect to auth for now
			// 	Either::Right(ok(
			// 		// req.into_response(HttpResponse::Found().header(actix_web::http::header::LOCATION, "/auth").finish().into_body())
			// 		req.into_response(HttpResponse::Found()
			// 			.header(actix_web::http::header::LOCATION, "/auth")
			// 			.finish().into_body()
			// 		)
			// 	))
			// },

			_ => {

				// all other paths require authentication

				match is_logged_in {

					false => {
						// Send 401, www-authenticate for basic

						Either::Right(ok(
						// req.into_response(HttpResponse::Found().header(actix_web::http::header::LOCATION, "/auth").finish().into_body())
							req.into_response(HttpResponse::Unauthorized()
								.header(actix_web::http::header::WWW_AUTHENTICATE, "Basic realm=\"Developer Portal\"")
								.finish().into_body()
							)
						))
					},

					true => {
						// logged in, proceed as requested
						Either::Left(self.service.call(req))
					}
				}
			}
		}
	}
}
