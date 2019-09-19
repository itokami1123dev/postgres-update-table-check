pub mod config;
pub mod checker;

use crate::config::create_postgres_conf;

fn main() {
    let conf = create_postgres_conf();
    checker::exec(conf);
}
