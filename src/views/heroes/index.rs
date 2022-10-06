use askama::Template;
use crate::models::hero::Hero;

#[derive(Template)]
#[template(path = "heroes/index.html")] 

struct ShowTemplate<'a> {
	heroes: &'a Vec<Hero>
}

pub fn render(heroes: &Vec<Hero>) -> String {
	let hello = ShowTemplate { heroes };
	hello.render().unwrap()
}
