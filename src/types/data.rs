use std::error::Error;
use crate::types::assignment::Assignment;
use crate::types::grade::Grade;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub assignments: Vec<Assignment>,
    pub grades: Vec<Grade>,
}

impl Data {
    pub fn empty() -> Self {
        Self {
            assignments: Vec::new(),
            grades: Vec::new(),
        }
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
        self.assignments.sort_by(|a, b| {
            if let (Some(a_date), Some(b_date)) = (a.date, b.date) {
                match a_date.date_naive().cmp(&b_date.date_naive()) {
                    Ordering::Equal => a.completed.cmp(&b.completed),
                    _ => a_date.cmp(&b_date),
                }
            } else if let Some(_) = a.date {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }

    pub fn update_assignments(&mut self, assignments: Vec<Assignment>) {
        for assignment in assignments {
            if let Some(a) = self.assignments.iter_mut().find(|a| a.html_url == assignment.html_url) {
                if !a.modified {
                    a.completed |= assignment.completed;
                }
                a.locked = assignment.locked;
                a.description = assignment.description;
                a.date = assignment.date;
                a.populate_links();
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

