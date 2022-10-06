pub mod service;
pub mod controllers;
pub mod models;
use actix_web::{App, HttpServer, get, web, HttpResponse, Responder};


// test
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
		.service(index)
        .service(web::scope("/heroes")
            .service(controllers::heroes::all_heroes)
            .service(controllers::heroes::get_hero)
        )
    })
    .workers(10)
    .keep_alive(15)
    .bind("127.0.0.1:8000")?
    .run()
    .await
}


#[get("/")]
pub async fn index() -> impl Responder {
	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(include_str!("./views/index.html"))
}