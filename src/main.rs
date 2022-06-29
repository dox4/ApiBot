use std::fs;

mod command;
mod db;
mod time;
mod http;

// check if first runnning
fn check_if_first_running() {
    use std::path::Path;
    let p = Path::new(db::DB_PATH);
    if !p.exists() {
        fs::create_dir_all(p).unwrap();
        db::init().unwrap();
    }
    // let version = env!("CARGO_PKG_VERSION");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_if_first_running();
    command::execute().await?;
    Ok(())
}
