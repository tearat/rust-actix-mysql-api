use std::fmt::{Display, Formatter, Result};

pub struct Row {
  pub element: String,
  text: String
}

impl Display for Row {
  fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "{}", self.text)
  }
}

pub fn parse_body(body: &String) -> Vec<Row> {
	let parsed: Vec<Row> = body.split("\n").map(|row| {
		match row.find("#") {
			Some(_) => Row {
				element: "Paragraph".to_string(),
				text: row.replace("#", "").trim().to_string()
			},
			None => Row {
				element: "Header".to_string(),
				text: row.to_string()
			},
		}
	}).collect();

  parsed
}