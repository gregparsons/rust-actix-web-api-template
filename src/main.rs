/*
	A working template for Actix-Web
	8/14/2014

	Ref:
	https://actix.rs/docs/getting-started/
*/

// use actix_web::dev::Service;

use actix_service::Service;
// use actix_web::{web, App};
use futures::future::FutureExt;

//logging
use std::env;
// #[macro_use]
// extern crate log;
// use log::Level;
// use log::Level::Info


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
	std::env::set_var("RUST_LOG", "easy_git_web=debug,actix_web=debug,actix_server=debug");
	env_logger::init();
	log::info!("starting main");
	log::debug!("starting main");
	log::error!("starting main");

	actix_web::HttpServer::new(||{
		actix_web::App::new()
			// simple middleware: https://actix.rs/docs/middleware/
			.wrap(actix_web::middleware::Logger::default())
			// .wrap(actix_web::middleware::new("%a %{User-Agent}i"))
			.wrap_fn(|req, srv|{
				println!("Path requested: {}", req.path());
				srv.call(req).map(|response| {
					println!("Response: {:?}", response);
					response
				})
			})
			.route("/", actix_web::web::get().to(index))
			.route("/say/{message01}/{message02}", actix_web::web::get().to(get_say_message))
			.route("/static/{filename:.*.html}", actix_web::web::get().to(get_static_file))
			.route("/info/{param1}", actix_web::web::get().to(get_info))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await

}
