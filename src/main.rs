use std::fs;

mod command;
mod db;
mod time;
mod http;
mod util;

// check if first runnning
fn check_if_first_running() {
    let p = util::app_home_dir();
    println!("{:?}, {:?}", p, p.exists());
    if !p.exists() {
        fs::create_dir_all(p).unwrap();
        db::init().unwrap();
    }
    // let version = env!("CARGO_PKG_VERSION");
}

fn main() {
    check_if_first_running();
    command::execute().unwrap();
}
