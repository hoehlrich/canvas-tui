use std::error::Error;
use crate::types::assignment::Assignment;
use crate::types::grade::Grade;
use crate::types::link::Link;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub assignments: Vec<Assignment>,
    pub grades: Vec<Grade>,
}

impl Data {
    pub async fn from_course_ids(course_ids: Vec<u32>, debug: bool) -> Result<Self, Box<dyn std::error::Error>> {
        if debug {
            println!("Fetching assignments...");
        }
        let assignments = crate::queries::assignments::query_assignments(&course_ids).await?;
        if debug {
            println!("Fetched {} assignments", assignments.len());
            println!("Fetching grades...");
        }
        let grades = crate::queries::grades::query_grades(&course_ids).await?;
        if debug {
            println!("Fetched {} grades", grades.len());
        }
        Ok(Self { assignments, grades })
    }

    pub fn serialize(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn serialize_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let data = self.serialize()?;
        std::fs::write(path, data)?;

        Ok(())
    }

    pub fn deserialize(data: &str) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn deserialize_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let data = std::fs::read_to_string(path)?;
        Self::deserialize(&data)
    }
}

