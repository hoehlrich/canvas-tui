use std::error::Error;
use std::path::Path;
use tokio;

mod types;
mod queries;
mod ui;

use types::data::Data;
use chrono::{Duration, DateTime};

async fn run_tui(path: String) -> Result<(), Box<dyn Error>> {
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

async fn serve(path: String) -> Result<(), Box<dyn Error>> {
    let course_ids = vec![72125, 71983, 72567, 71447, 72767]; // Henry course ids
    // let course_ids = vec![71983, 72567, 71415, 72131]; // Shaurya course ids
    let mut data = match Data::deserialize_from_file(&path) {
        Ok(data) => data,
        Err(_) => {
            println!("No data found, fetching from course ids...");
            match Data::from_course_ids(course_ids.clone(), true).await {
                Ok(d) => {
                    d.serialize_to_file(&path)?;
                    d
                }
                Err(e) => {
                    eprintln!("Error fetching data: {}", e);
                    return Err(e);
                }
            }
        }
    };

    // refresh data every minute
    let mut last_update: DateTime<chrono::Utc> = chrono::DateTime::UNIX_EPOCH;
    loop {
        if chrono::Utc::now() - last_update > Duration::seconds(60) {
            println!("Refreshing data...");
            match Data::from_course_ids(course_ids.clone(), true).await {
                Ok(d) => {
                    data.assignments = d.assignments;
                    data.grades = d.grades;
                    data.serialize_to_file(&path)?;
                    last_update = chrono::Utc::now();
                }
                Err(e) => eprintln!("Error fetching data: {}", e),
            }
            println!("Data refreshed.");
        }
    }
}

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
    // source envvars
    dotenv::dotenv().ok();

    let args = std::env::args().collect::<Vec<_>>();
    let path = std::env::var("HOME")? + "/.local/share/canvas-tui/data.json";
    create_path(&path)?;

    if let Some(a) = args.get(1) {
        match a.as_str() {
            "tui" => run_tui(path).await?,
            "serve" => serve(path).await?,
            _ => {
                eprintln!("Invalid argument. Use 'tui' or 'serve'.");
                return Ok(());
            }
        }
    } else {
        println!("No argument provided: serving data");
        serve(path).await?;
    }

    Ok(())
}
