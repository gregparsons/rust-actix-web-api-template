/*
	A working template for Actix-Web
	8/14/2014

	Ref:
	https://actix.rs/docs/getting-started/
*/

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	actix_web::HttpServer::new(||{
		actix_web::App::new()
			.route("/", actix_web::web::get().to(index))
			.route("/say/{message01}/{message02}", actix_web::web::get().to(get_say_message))
			.route("/static/{filename:.*.html}", actix_web::web::get().to(get_static_file))
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await

}
