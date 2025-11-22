use std::error::Error;
use std::path::Path;
use tokio;

mod types;
mod queries;
mod ui;

use types::data::Data;

static DATA_EXT: &str = "/.local/share/canvas-tui/";
static CONFIG_EXT: &str = "/.config/canvas-tui/";

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
    // Load config
    let home = std::env::var("HOME")?;
    let settings = config::Config::builder()
        .add_source(config::File::with_name(format!("{}{}config", home, CONFIG_EXT).as_str()))
        .build()?;

    let course_ids = settings
        .get_array("course_ids")?
        .iter()
        .map(|v| v.clone().try_deserialize::<u32>())
        .collect::<Result<Vec<u32>, config::ConfigError>>()?;

    let data_dir = match settings.get_string("data_dir") {
        Ok(v) => {
            if v.starts_with('~') {
                format!("{}{}", home, &v[1..])
            } else {
                v
            }
        },
        Err(_) => format!("{}{}", home, DATA_EXT)
    };


    // Load data 
    println!("{}", data_dir);
    dotenv::from_filename(format!("{}{}.env", home, CONFIG_EXT))?;
    let data_path = data_dir.to_string() + "data.json";
    create_path(&data_path)?;

    let data = match Data::deserialize_from_file(&data_path) {
        Ok(data) => data,
        Err(_) => Data::empty(),
    };

    // Run
    let res = ui::run(data_path, course_ids, data).await;
    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
