/*
	A working template for Actix-Web
	8/14/2014

	Ref:
	https://actix.rs/docs/getting-started/
*/

//logging
use actix_web::HttpResponse;
// use futures::task::{Context, Poll};

// Authentication
const PASSWORD:&str = "topsecret";
pub const TOKEN_SECRET:&str = "asdfasdf";

// Middleware
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
// use futures::Future;
use std::future::Future;

// use jsonwebtoken::DecodingKey;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Login{
	password:String,
}


// There are two steps in middleware processing.
// 1. Middleware initialization: the middleware factory gets called with the
//    next service in chain as a parameter.
// 2. Middleware's call method gets called with a normal request.
pub struct TransformLogin;

// The middleware factory is a `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response body
impl<S, B> Transform<S> for TransformLogin
	where
		S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
		B: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type InitError = ();
	// Here's where the middleware call() effectively gets "called"
	type Transform = LoginMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {

		ok(LoginMiddleware { service })

	}
}

pub struct LoginMiddleware<S> {
	service: S,
}

/// Implement the middleware service
impl<S, B> Service for LoginMiddleware<S>
	where
		S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
		S::Future: 'static,
		B: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}

	// The central point of the login process. Block the service/request here if needed.
	fn call(&mut self, req: ServiceRequest) -> Self::Future {
		log::debug!("[call] Request received: {}", req.path());

		if req.path() == "/login"{
			log::debug!("[call] /login requested ")
		}
		else {
			log::debug!("[call] /login not requested ")
		}
		// if req.path() == "/login" {
		// 	// proceed, don't need to be authenticated to head to the login page
		// 	let fut = self.service.call(req);
		// 	Box::pin(async move {
		// 		let res = fut.await?;
		// 		Ok(res)
		// 	})
		// } else {
		// 	// check if there's an HTTP AUTHORIZATION header
		// 	if let Some(header_value) = req.headers().get(actix_web::http::header::AUTHORIZATION){
		// 		// basically, match on Some(header_value)
		// 		// Strip off 'bearer'
		// 		let token = header_value.to_str().unwrap().replace("Bearer", "");
		// 		let mut validation = jsonwebtoken::Validation::default();
		// 		// validate the jwt exp field? no. our logins don't expire.
		// 		validation.validate_exp = false;
		// 		// decode the JWT
		// 		if let Ok(_) = jsonwebtoken::decode::<Claims>(&token.trim(), &DecodingKey::from_secret(TOKEN_SECRET.as_ref()), &validation){
		// 			// Either::A(self.service.call(req))
		// 			let fut = self.service.call(req);
		// 			Box::pin(async move {
		// 				let res = fut.await?;
		// 				Ok(res)
		// 			})
		// 		} else {
		// 			// Either::B(ok(req.into_response(HttpResponse::Unauthorized().finish().into_body())))
		// 			let fut = req.into_response(HttpResponse::Unauthorized().finish().into_body());
		// 			Box::pin(async move {
		// 				let res = fut.await?;
		// 				Ok(res)
		// 			})
		// 		}
		// 	} else {
		// 		// ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
		// 		// if there's no AUTHORIZATION header, bail.
		// 		let fut = req.into_response(HttpResponse::Unauthorized().finish().into_body());
		// 		Box::pin(async move {
		// 			let res = fut.await?;
		// 			Ok(res)
		// 		})
		// 		// Either::B(ok(req.into_response(HttpResponse::Unauthorized().finish().into_body())))
		// 	}
		// }


		let fut = self.service.call(req);

		Box::pin(async move {
			let res = fut.await?;

			log::debug!("[SayHiMiddleware service.call] Response going out.");
			Ok(res)
		})
	}
}





// fn authorized() -> impl actix_web::Responder {
// 	format!("Congrats. You're in.")
// }

fn login(login: actix_web::web::Json<Login>) -> actix_web::HttpResponse {
	//TODO: proper security
	// Do the authentication

	log::debug!("Logging in...");

	if &login.password == PASSWORD {
	//if 1 == 1 {
		log::debug!("Logged in.");
		let claims = Claims{
			user_id:"1".into()
		};

		// send a bearer token if authenticated
		jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, & jsonwebtoken::EncodingKey::from_secret(TOKEN_SECRET.as_ref())) // .unwrap()
			.map(|token| {
				actix_web::HttpResponse::Ok()
					.header(actix_web::http::header::AUTHORIZATION, format!("Bearer {}", token))
					.finish()
			})
			.unwrap_or(HttpResponse::InternalServerError().into())
	 } else {
		log::debug!("Unauthorized.");
		actix_web::HttpResponse::Unauthorized().into()
	}
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims{
	pub user_id:String,
}






/// GET the index.html file from the static directory.
async fn index() -> actix_web::Result<actix_files::NamedFile>{
	let path_string:String = format!("static/index.html");
	let path_buf:std::path::PathBuf = std::path::PathBuf::from(&path_string);
	Ok(actix_files::NamedFile::open(&path_buf)?)
}

/// GET a file from the static directory.
async fn get_static_file(req: actix_web::HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
	let filename = req.match_info().query("filename").parse::<String>().unwrap();
	let path_string:String = format!("static/{}",&filename);
	let path_buf:std::path::PathBuf = std::path::PathBuf::from(&path_string);
	Ok(actix_files::NamedFile::open(&path_buf)?)
}

#[derive(serde::Deserialize)]
struct HttpParamMessage {
	message01:String,
	message02:String,
}

/// GET a message containing the two segments following a "say" segment.
async fn get_say_message(mesg: actix_web::web::Path<HttpParamMessage>) -> actix_web::HttpResponse {
	actix_web::HttpResponse::Ok().body(format!("I'd like to say: {} and {}", &mesg.message01, &mesg.message02))
}

async fn get_info(req:actix_web::web::HttpRequest) -> actix_web::HttpResponse {
	let mut resp_string = String::from("");
	resp_string.push_str("<html><body>");
	resp_string.push_str(format!("path: {}", req.path()).as_str());
	let params = req.match_info();
	resp_string.push_str(format!("<br>{:?}", &params).as_str());
	resp_string.push_str("</body></html>");
	actix_web::HttpResponse::Ok().body(&resp_string)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	// start logging;
	// environment variable controls the logging level
	// https://docs.rs/env_logger/0.7.1/env_logger/index.html
	// std::env::set_var("RUST_LOG", "easy_git_web=debug,actix_web=debug,actix_server=debug");
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();
	log::info!("starting main");
	log::debug!("starting main");
	log::error!("starting main");

	// web server
	actix_web::HttpServer::new(||{
		actix_web::App::new()
			// logging middleware: https://actix.rs/docs/middleware/
			.wrap(actix_web::middleware::Logger::default())
			// .wrap_fn(|req, srv|{
			// 	// sample middleware
			// 	println!("Path requested: {}", req.path());
			// 	srv.call(req).map(|response| {
			// 		println!("Response: {:?}", response);
			// 		response
			// 	})
			// })
			.wrap(TransformLogin)
			.route("/", actix_web::web::get().to(index))
			.route("/login", actix_web::web::get().to(login))
			.route("/say/{message01}/{message02}", actix_web::web::get().to(get_say_message))
			.route("/static/{filename:.*.html}", actix_web::web::get().to(get_static_file))
			.route("/info/{param1}", actix_web::web::get().to(get_info))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await

}
