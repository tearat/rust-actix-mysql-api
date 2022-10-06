use crate::service::connector::get_connect;
use crate::service::responses::ResponseError;
use crate::models::hero::{Hero, HeroExtended};
use actix_web::{get, web, HttpResponse, Responder};
use mysql::prelude::Queryable;
use serde::Serialize;
use crate::views::heroes::show;

#[derive(Serialize)]
pub struct ResponseSuccess {
    result: bool,
    items: Vec<Hero>,
}

#[get("")]
pub async fn all_heroes() -> impl Responder {
    let mut conn = get_connect().expect("Failed to establish mysql connection.");

    let sql = "select id, title from heroes limit 5";

    let items = conn.query_map(sql, |(id, title)| {
        Hero { id, title }
    })
    .expect("Query failed.");

    HttpResponse::Ok().json(items)
}


#[get("/{id}")]
pub async fn get_hero(web::Path(id): web::Path<u32>) -> impl Responder {
    let mut conn = get_connect().expect("Failed to establish mysql connection.");

    let sql = format!("select id, title, body from heroes where id = {} limit 1", &id.to_string());

    let items = conn.query_map(sql, |(id, title, body)| {
        HeroExtended { id, title, body }
    })
    .expect("Query failed.");

    if items.len() == 1 {
        let first_hero = &items[0];
        HttpResponse::Ok().content_type("text/html; charset=utf-8").body(show::render(first_hero))
    } else {
        HttpResponse::NotFound().json(ResponseError { error: String::from("Hero not found") })
    }
}