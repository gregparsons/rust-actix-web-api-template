/*
	A working template for Actix-Web with:
		- logging
		- basic authentication via authorization headers
		-
	8/14/2014

	Ref:
	https://actix.rs/docs/getting-started/
*/

// try: https://github.com/actix/examples/blob/master/middleware/src/redirect.rs
mod redirect;

//logging
// use jsonwebtoken::EncodingKey;

// Authentication: obviously don't use these in real life
const FAKE_USER_PASSWORD:&str = "topsecret";
// const TOKEN_SECRET:&str = "asdfasdf";

/// [/auth] for future use converting to an HTML-based login form
/// Not currently needed as all functionality recommended in various books and sites has been moved to the middleware
/// If username and password are included in an AUTHORIZATION header
/// ref: https://www.jamesbaum.co.uk/blether/creating-authentication-middleware-actix-rust-react/
async fn auth(req: actix_web::HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
	log::debug!("[auth] request headers: {:?}", req.headers());
	let path_buf:std::path::PathBuf = std::path::PathBuf::from("static/auth.html");
	Ok(actix_files::NamedFile::open(&path_buf)?)
	//https://actix.rs/actix-web/actix_web/dev/struct.HttpResponseBuilder.html
	//https://tools.ietf.org/html/rfc7617
	// HttpResponse::Unauthorized().header(actix_web::http::header::WWW_AUTHENTICATE, "Basic realm=\"WallyWorld\"").finish()
}


/// [/]
async fn index() -> actix_web::Result<actix_files::NamedFile>{
	let path_string:String = format!("static/index.html");
	let path_buf:std::path::PathBuf = std::path::PathBuf::from(&path_string);
	Ok(actix_files::NamedFile::open(&path_buf)?)
}


/// GET a file from the static directory.
/// Insecure
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


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	// start logging;
	// environment variable controls the logging level
	// https://docs.rs/env_logger/0.7.1/env_logger/index.html
	// std::env::set_var("RUST_LOG", "easy_git_web=debug,actix_web=debug,actix_server=debug");
	std::env::set_var("RUST_LOG", "debug");
	env_logger::init();

	// web server
	// logging middleware: https://actix.rs/docs/middleware/
	actix_web::HttpServer::new(||{
		actix_web::App::new()
			.wrap(actix_web::middleware::Logger::default())
			// Low boiler-plate middleware
			// .wrap_fn(|req, srv|{
			// 	// sample middleware
			// 	println!("Path requested: {}", req.path());
			// 	srv.call(req).map(|response| {
			// 		println!("Response: {:?}", response);
			// 		response
			// 	})
			// })
			.wrap(redirect::CheckLogin)
			.route("/", actix_web::web::get().to(index))
			.route("/auth", actix_web::web::get().to(auth))
			.route("/say/{message01}/{message02}", actix_web::web::get().to(get_say_message))
			.route("/static/{filename:.*.html}", actix_web::web::get().to(get_static_file))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await

}
