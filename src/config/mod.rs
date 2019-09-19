extern crate rpassword;

use std::env;
use std::io;

pub struct PostgresConf {
  pub pg_host:String,
  pub pg_port:String,
  pub pg_database:String,
  pub pg_user:String,
  pub pg_pass:String,
  pub pg_option_time_zone:String,
  pub pg_option_exclude_table:String,
}

impl PostgresConf {
  pub fn new(host:String, port:String, db:String, user:String, pass:String, ex:String) -> Self {
      PostgresConf {
        pg_host:host,
        pg_port:port,
        pg_database:db,
        pg_user:user,
        pg_pass:pass,
        pg_option_time_zone:"Asia/Tokyo".to_string(),
        pg_option_exclude_table:ex,
      }
  }
}

pub fn create_postgres_conf() -> PostgresConf {
  let pg_host = String::from("PGHOST");
  let pg_port = String::from("PGPORT");
  let pg_database = String::from("PGDATABASE");
  let pg_user = String::from("PGUSER");
  let pg_option_exclude_table = String::from("EXCLUDE_TABLE");

  let host = match env::var(pg_host) {
    Ok(val) => val,
    Err(_) => {
      println!("host: ?");
      let mut input_host = String::new();
      io::stdin()
      .read_line(&mut input_host)
      .expect("Failed to read line");
      input_host.trim().to_string()
    },
  };

  let port = match env::var(pg_port) {
    Ok(val) => val,
    Err(_) => {
      println!("port: ? ");
      let mut input_port = String::new();
      io::stdin()
      .read_line(&mut input_port)
      .expect("Failed to read line");
      input_port.trim().to_string()
    },
  };

  let db = match env::var(pg_database) {
    Ok(val) => val,
    Err(_) => {
      println!("database: ? ");
      let mut input_database = String::new();
      io::stdin()
      .read_line(&mut input_database)
      .expect("Failed to read line");
      input_database.trim().to_string()
    },
  };

  let user = match env::var(pg_user) {
    Ok(val) => val,
    Err(_) => {
      println!("user: ? ");
      let mut input_user = String::new();
      io::stdin()
      .read_line(&mut input_user)
      .expect("Failed to read line");
      input_user.trim().to_string()
    },
  };

  let exclude = match env::var(pg_option_exclude_table ) {
    Ok(val) => val,
    Err(_) => "'___dummy___'".to_string(),
  };

  println!("======================================");
  println!(
    "host = {}, port = {}, database = {}, db_user = {}, exclude_table = {} ",
    host, port, db, user, exclude 
  );
  println!("======================================");

  let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
  let conf = PostgresConf::new(host,port,db,user,pass, exclude);

  return conf;
}