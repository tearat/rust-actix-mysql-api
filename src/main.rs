use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use actix_web::{App, HttpServer, get, web, HttpResponse, Responder};
use std::fs::File;
use std::io::prelude::*;

// TODO
// #[derive(Deserialize)]
// pub struct Request {
//     id: u64,
// }

#[derive(Deserialize, Debug)]
struct Config {
	mysql: String,
}

#[derive(Serialize)]
pub struct ResponseSuccess {
    result: bool,
    items: Vec<Hero>,
}

#[derive(Serialize)]
pub struct ResponseError {
    error: String,
}

#[derive(Serialize)]
struct Hero {
    id: u64,
    title: String
}

// test
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
		.service(index)
        .service(web::scope("/heroes")
            .service(all_heroes)
            .service(get_hero)
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


#[get("")]
pub async fn all_heroes() -> impl Responder {
    let mut conn = get_conn().expect("Failed to establish mysql connection.");

    let sql = "select id, title from heroes limit 5";

    let items = conn.query_map(sql, |(id, title)| {
        Hero { id, title }
    })
    .expect("Query failed.");

    HttpResponse::Ok().json(ResponseSuccess { result: true, items })
}


#[get("/{id}")]
pub async fn get_hero(web::Path(id): web::Path<u32>) -> impl Responder {
    let mut conn = get_conn().expect("Failed to establish mysql connection.");

    let mut sql = String::from("select id, title from heroes where id = ");
    sql += &id.to_string();
    sql += " limit 1";

    let items = conn.query_map(sql, |(id, title)| {
        Hero { id, title }
    })
    .expect("Query failed.");

    if items.len() > 0 {
        HttpResponse::Ok().json(ResponseSuccess { result: true, items })
    } else {
        HttpResponse::NotFound().json(ResponseError { error: String::from("Hero not found") })
    }
}


fn get_conn() -> Result<PooledConn> {
    let config = get_config().expect("Failed to load config.");
    let url = &config.mysql;
	let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn()?;
    
    Ok(conn)
}


fn get_config() -> Result<Config> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents).unwrap();

    Ok(config)
}
