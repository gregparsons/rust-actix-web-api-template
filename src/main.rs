/*
	A working template for Actix-Web
	8/14/2014

	Ref:
	https://actix.rs/docs/getting-started/
*/

// try: https://github.com/actix/examples/blob/master/middleware/src/redirect.rs
mod redirect;

//logging
use actix_web::HttpResponse;
use jsonwebtoken::EncodingKey;

// Authentication
const PASSWORD:&str = "topsecret";
pub const TOKEN_SECRET:&str = "asdfasdf";





// fn authorized() -> impl actix_web::Responder {
// 	format!("Congrats. You're in.")
// }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Login{
	password:String,
}

/// Require http basic authentication via server-sent WWW-AUTHENTICATE header
/// If username and password are included in an AUTHORIZATION header
/// ref: https://www.jamesbaum.co.uk/blether/creating-authentication-middleware-actix-rust-react/
async fn auth(req: actix_web::HttpRequest) -> HttpResponse {
	log::debug!("[get_creds]");
	log::debug!("[get_creds] request headers: {:?}", req.headers());
	let auth_header = req.headers().get("authorization");

	match auth_header {

		None => {
			// No www-authenticate header so tell browser to send it
			log::debug!("[get_creds] authorization header: None");
			HttpResponse::Unauthorized()
				.header(actix_web::http::header::WWW_AUTHENTICATE, "Basic realm=\"Developer Portal\"")
				.finish()
		},

		Some(auth_header) => {
			// An HTTP AUTHORIZATION header was received. Extract username and password.
			// (1) Check for "Basic " followed by base64 encoded "username:password"
			// (2) Decode base64, check password (TODO: real authentication)
			// (3) Create a JWT token if the password is good.
			// (4) Test: Confirm the JWT token be decoded
			// (5) Redirect to home, or original path perhaps
			// encoding of the username and password separated by a colon.
			log::debug!("[get_creds] raw authorization header: {:?}", &auth_header.to_str());
			// TODO: make case insensitive, confirm per RFC
			// TODO: Check for other kinds of authentication headers besides BASIC
			let auth_header:String = auth_header.to_str().unwrap().replace("Basic ", "");
			log::debug!("[get_creds] username/password base64 encoded: {}", auth_header.as_str());
			let auth_header:Vec<u8> = auth_header.into_bytes();
			let auth_header:Vec<u8> = base64::decode(auth_header).unwrap();
			let auth_header:String = String::from_utf8(auth_header.to_owned()).unwrap();

			// log::debug!("[get_creds] base64 auth header: {}", auth_header.to_str().unwrap());
			// log::debug!("[get_creds] decoded auth header: {}", auth_utf8);

			let credentials:Vec<&str> = auth_header.split(":").collect();
			let username = credentials[0];
			let password = credentials[1];

			log::debug!("[get_creds] username: {}", credentials[0]);
			log::debug!("[get_creds] password: {}", credentials[1]);

			if password == PASSWORD {
				log::debug!("[get_creds] Success. Password matches.");

				// Now, issue a token only this site could have issued.
				let user_claim = Claims{
					user_id: "1".to_owned(),
				};

				log::debug!("[get_creds] Creating bearer header and done.");

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

					// {
					// 	HttpResponse::Found()
					// 		.header(actix_web::http::header::LOCATION, "/secret")
					// 		.finish()
					//
					//
					// 	// HttpResponse::Ok()
					// 	// .header(actix_web::http::header::LOCATION, "/")
					// 	// .header(actix_web::http::header::AUTHORIZATION, format! ("Bearer {}", token))
					// 	// .finish()
					// }) .unwrap_or(HttpResponse::InternalServerError().into())





				// let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(),
				// 							   &user_claim,
				// 							   &jsonwebtoken::EncodingKey::from_secret(TOKEN_SECRET.as_ref())).unwrap();
				// log::debug!("[get_creds] jwt: {}",&jwt);
				//
				// // JWT decode test
				// let mut validation = jsonwebtoken::Validation::default();
				// validation.validate_exp = false;
				// let decoded_token_test = jsonwebtoken::decode::<Claims>(&jwt, &jsonwebtoken::DecodingKey::from_secret(TOKEN_SECRET.as_ref()), &validation);
				// if let Ok(token_data) = decoded_token_test {
				// 	log::debug!("[get creds] decoded jwt user ID: {:?}", &token_data.claims.user_id);
				// 	assert_eq!(&token_data.claims.user_id, &user_claim.user_id)
				// }
				//
				// log::debug!("[get_creds] Creating bearer header and done.");
				//
				// // Encode the JWT, put it in the header and redirect again
				// HttpResponse::Ok()
				// 	.header(actix_web::http::header::LOCATION, "/")
				// 	.header(actix_web::http::header::AUTHORIZATION, format! ("Bearer {}", jwt))
				// 	.finish()


				//
				// HttpResponse::Found()
				// 	.header(actix_web::http::header::LOCATION, "/")
				// 	.finish()
				// 	.into_body()


			} else {

				// Password didn't match. Denied. Or try again.
				log::debug!("[get_creds] Password didn't match. Denied. Routing back to /auth");

				// HttpResponse::Found()
				// 	.header(actix_web::http::header::LOCATION, "/auth")
				// 	.finish()
				// 	.into_body()
				HttpResponse::Unauthorized().into()

			}



			// HttpResponse::Found()
			// 	.header(actix_web::http::header::LOCATION, "/")
			// 	.finish()
			// 	.into_body()
		}
	}



	//https://actix.rs/actix-web/actix_web/dev/struct.HttpResponseBuilder.html
	//https://tools.ietf.org/html/rfc7617
	// HttpResponse::Unauthorized().header(actix_web::http::header::WWW_AUTHENTICATE, "Basic realm=\"WallyWorld\"").finish()

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

/****************** TEST from book REMOVE *************/
async fn authed() -> impl actix_web::Responder {
	format!("Congrats, you are authenticated")
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
			// .wrap(TransformLogin)
			.wrap(redirect::CheckLogin)
			.route("/", actix_web::web::get().to(index))
			.route("/secret", actix_web::web::get().to(authed))
			.route("/auth", actix_web::web::get().to(auth))
			.route("/say/{message01}/{message02}", actix_web::web::get().to(get_say_message))
			.route("/static/{filename:.*.html}", actix_web::web::get().to(get_static_file))
			.route("/info/{param1}", actix_web::web::get().to(get_info))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await

}
