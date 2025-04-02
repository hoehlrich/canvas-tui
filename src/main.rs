use std::error::Error;
use tokio;

mod types;
mod queries;
mod ui;

use types::data::Data;
use chrono::{Duration, DateTime};

async fn run_tui() -> Result<(), Box<dyn Error>> {
    let data = Data::deserialize_from_file("data.json")?;
    let res = ui::run(data).await;
    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

async fn serve() -> Result<(), Box<dyn Error>> {
    let course_ids = vec![72125, 71983, 72567, 71447, 72767]; // Henry course ids
    // let course_ids = vec![71983, 72567, 71415, 72131]; // Shaurya course ids
    let mut data = match Data::deserialize_from_file("data.json") {
        Ok(data) => data,
        Err(_) => {
            println!("No data found, fetching from course ids...");
            match Data::from_course_ids(course_ids.clone(), true).await {
                Ok(d) => {
                    d.serialize_to_file("data.json")?;
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
                    data.serialize_to_file("data.json")?;
                    last_update = chrono::Utc::now();
                }
                Err(e) => eprintln!("Error fetching data: {}", e),
            }
            println!("Data refreshed.");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // source envvars
    dotenv::dotenv().ok();

    let args = std::env::args().collect::<Vec<_>>();

    if let Some(a) = args.get(1) {
        match a.as_str() {
            "tui" => run_tui().await?,
            "serve" => serve().await?,
            _ => {
                eprintln!("Invalid argument. Use 'tui' or 'serve'.");
                return Ok(());
            }
        }
    } else {
        println!("No argument provided: serving data");
        serve().await?;
    }

    Ok(())
}
