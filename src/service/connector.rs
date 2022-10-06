use std::fs::File;
use std::io::prelude::*;
use mysql::{PooledConn,Result,Pool};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct Config {
	mysql: String,
}

pub fn get_connect() -> Result<PooledConn> {
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