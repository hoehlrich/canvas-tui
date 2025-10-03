use chrono::{DateTime, Duration, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::types::link::Link;
use select::document::Document;
use select::predicate::Name;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Assignment {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub date: Option<DateTime<FixedOffset>>,
    pub course: String,
    pub completed: bool,
    pub custom: bool,
    pub modified: bool,
    pub locked: bool,
    pub links: Vec<Link>,
}

impl Assignment {
    pub fn new(
        name: String,
        nickname: Option<String>,
        description: Option<String>,
        html_url: String,
        datestring: Option<String>,
        course: String,
        completed: bool,
        locked: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let course = if let Some(nickname) = nickname {
            nickname
        } else {
            course
        };

        let date = if let Some(datestring) = datestring {
            Some(DateTime::parse_from_rfc3339(&datestring)?)
        } else {
            None
        };

        let mut a = Self {
            name,
            description,
            html_url,
            date,
            course,
            completed,
            custom: false,
            modified: false,
            locked,
            links: vec![],
        };
        a.populate_links();
        Ok(a)
    }

    pub fn empty() -> Self {
        let today = chrono::Local::now().date_naive();
        let time = chrono::NaiveTime::from_hms_opt(23, 59, 0).unwrap();
        let naive_dt = today.and_time(time);
        let now = chrono::Local::now();
        let offset = now.offset();
        let dt: chrono::DateTime<FixedOffset> = offset.from_local_datetime(&naive_dt).unwrap();
        

        Self {
            name: String::new(),
            description: None,
            html_url: String::new(),
            date: Some(dt),
            course: String::new(),
            completed: false,
            custom: true,
            modified: false,
            locked: false,
            links: vec![]
        }
    }

    pub fn populate_links(&mut self) {
        let description = match &self.description {
            Some(v) => v,
            None => return,
        };
        self.links = Document::from(description.as_str())
            .find(Name("a"))
            .map(|n| {
                let url = match n.attr("href") {
                    Some(v) => v,
                    None => return None,
                };
                let text = n.text();
                Some(Link::new(url.to_string(), text))
            })
            .filter_map(|x| x)
            .collect::<Vec<Link>>();
    }

    // Make the due date one day later
    pub fn increment_due_date(&mut self) {

        match self.date {
            Some(dt) => self.date = Some(dt + Duration::days(1)),
            None => return,
        }
    }

    // Make the due date one day earlier (bounded by today)
    pub fn decrement_due_date(&mut self) {
        match self.date {
            Some(dt) => {
                let new_dt = dt - Duration::days(1);
                if new_dt > chrono::Local::now() {
                    self.date = Some(dt - Duration::days(1));
                }
            },
            None => return,
        }
    }

}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(date) = self.date {
            write!(
                f,
                "{} - {} ({})",
                self.course,
                self.name,
                date.format("%A %d")
            )
        } else {
            write!(f, "{} - {} (No due date)", self.course, self.name)
        }
    }
}
