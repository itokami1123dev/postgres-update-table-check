extern crate postgres;

use crate::config::PostgresConf;

use postgres::{Connection, TlsMode};
use std::collections::HashMap;
use std::io;
use std::time::Instant;

struct SqlResult {
  relname: String,
  upd: String,
}

pub fn exec(conf:PostgresConf){
  let mut map: HashMap<String, String> = HashMap::new();
  let mut begin_time = Instant::now();
  loop {
    println!("\n");
    
    let con_url = format!(
      "postgres://{}:{}@{}:{}/{}",
      conf.pg_user, conf.pg_pass, conf.pg_host, conf.pg_port, conf.pg_database
    );
    let conn = Connection::connect(con_url, TlsMode::None).unwrap();
    
    &conn.query("SET TIME ZONE 'Asia/Tokyo'; ", &[]).unwrap();

          let sql1 = format!("
     SELECT
       relname,
       n_tup_ins || '_' || n_tup_upd || '_' || n_tup_del || '_' || n_tup_hot_upd || '_' || n_live_tup AS upd
     FROM
       pg_stat_user_tables 
     WHERE
       relname NOT IN ({})
     ORDER BY
       relname ", conf.pg_option_exclude_table).to_string();
    // println!("sql1 = {}", sql1);
    for row in &conn.query(&sql1 ,&[]).unwrap() {

      let r = SqlResult {
        relname: row.get(0),
        upd: row.get(1),
      };


      if map.contains_key(&r.relname) {
        let u = map.get(&r.relname).unwrap();
        if u.as_str() != r.upd.as_str() {
          println!("\n");
          println!("-----");
          println!("{}", r.relname);
          println!("-----");

          let sql2 = format!("
          SELECT column_name FROM information_schema.\"columns\" WHERE table_name = '{}' ORDER BY ordinal_position
          ", r.relname).to_string();

          let mut select_query: String = " SELECT ".to_string();

          let mut i = 0;
          for row2 in &conn.query(&sql2, &[]).unwrap() {
            if i > 0 {
              select_query.push_str(" , ");
            }
            let tmp: String = row2.get(0);
            select_query.push_str(format!(" '' || {} AS {} ", tmp, tmp).as_str());
            i = i + 1;
          }
          select_query.push_str(format!(" FROM {} ", r.relname).as_str());

          let sql_x = format!("
          SELECT column_name FROM information_schema.\"columns\" WHERE table_name = '{}' AND data_type LIKE '%timestamp%'
          ", r.relname).to_string();
          
          select_query.push_str(" WHERE ");

          let mut i = 0;
          for row_x in &conn.query(&sql_x, &[]).unwrap() {
            if i > 0 {
              select_query.push_str(" OR ");
            }
            let tmp: String = row_x.get(0);
            let end_time = Instant::now();
            select_query.push_str(
              format!(
                " {}  >= (NOW() - INTERVAL '{} seconds')::TIMESTAMP ",
                tmp,
                (end_time - begin_time).as_secs()
              )
              .as_str(),
            );
            i = i + 1;
          }
          
          select_query.push_str(" LIMIT 10 ; ");

          for row3 in &conn.query(&select_query, &[]).unwrap() {
            for col in row3.columns() {
              print!("{}~", col.name());
            }
            print!("\n");
            let mut count = 0;
            for col in row3.columns() {
              let tp: String = col.type_().to_string();
              match tp.as_str() {
                "bytea" => {
                  print!("(bytea)~");
                },
                _ => {
                  // print!("tp.as_str() = {} \n", tp.as_str());
                  let x: Option<String> = row3.get(count);
                  print!("{}~", match x {
                    Option::Some(val) => val,
                    Option::None => "(null)".to_string()
                  });
                }
              }
              count = count + 1;
            }
            print!("\n");
          }
        }
      }

      map.insert(r.relname, r.upd);
    }

    begin_time = Instant::now();
    println!("--");
    println!("hit enter! (q: quit)");
    let mut command = String::new();
    io::stdin()
      .read_line(&mut command)
      .expect("Failed to read line");
    let s = command.trim().to_string();
    if &*s == "q" {
      println!("end");
      break;
    }
  }
}