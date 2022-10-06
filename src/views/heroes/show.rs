use askama::Template;
use crate::models::hero::HeroExtended;
use crate::service::body_parser::{Row, parse_body};

#[derive(Template)]
#[template(path = "heroes/show.html")] 

struct ShowTemplate<'a> {
	id: &'a str, 
	title: &'a str,
	body: &'a Vec<Row>
}

pub fn render(hero: &HeroExtended) -> String {
	let id = &hero.id.to_string();
	let title = &hero.title;
	let body = &parse_body(&hero.body);

	let hello = ShowTemplate { id, title, body };
	hello.render().unwrap()
}
