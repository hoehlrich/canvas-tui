use std::{error::Error, ops::Not};
use crate::types::assignment::Assignment;
use crate::types::grade::Grade;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub assignments: Vec<Assignment>,
    pub grades: Vec<Grade>,
    pub course_ids: Vec<u32>,
}

impl Data {
    pub fn empty() -> Self {
        Self {
            assignments: Vec::new(),
            grades: Vec::new(),
            course_ids: Vec::new(),
        }
    }

    pub async fn from_course_ids(course_ids: Vec<u32>, debug: bool) -> Result<Self, Box<dyn Error>> {
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
        Ok(Self { assignments, grades, course_ids})
    }

    pub fn remove_past_assignments(&mut self) {
        let now = chrono::Utc::now();
        self.assignments.retain(|a| {
            if let Some(date) = a.date {
                date > now
            } else {
                true
            }
        });
    }

    pub fn sort_assignments(&mut self) {
        self.assignments.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn update_assignments(&mut self, assignments: Vec<Assignment>) {
        for assignment in assignments {
            if let Some(a) = self.assignments.iter_mut().find(|a| a.html_url == assignment.html_url) {
                if !a.modified {
                    a.completed |= assignment.completed;
                }
            } else {
                self.assignments.push(assignment);
            }
        }
        self.sort_assignments();
    }

    pub fn get_number_incomplete(&self) -> usize {
        self.assignments.iter().filter(|a| !a.completed).count()
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

