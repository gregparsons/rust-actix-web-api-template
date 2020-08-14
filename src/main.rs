/*
	A working template for Actix-Web
	8/14/2014

	Ref:
	https://actix.rs/docs/getting-started/
*/

use actix_web::HttpResponse;

async fn get_say_hello() -> impl Responder {
	HttpResponse::Ok().body("Hello World.")
}



fn main() {

	


}
