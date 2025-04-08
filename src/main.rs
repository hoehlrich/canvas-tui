use std::error::Error;
use std::path::Path;
use tokio;

mod types;
mod queries;
mod ui;

use types::data::Data;

fn create_path(pathstr: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(pathstr);
    if !path.exists() {
        let parent = match path.parent() {
            Some(p) => p,
            None => return Err("Invalid path".into()),
        };
        std::fs::create_dir_all(parent)?;
        std::fs::File::create(path)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let path = std::env::var("HOME")? + "/.local/share/canvas-tui/data.json";
    create_path(&path)?;

    let data = match Data::deserialize_from_file(&path) {
        Ok(data) => data,
        Err(_) => Data::empty(),
    };
    let res = ui::run(data, path).await;
    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
