use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Grade {
    pub course: String,
    pub grade: f64,
    course_id: u32,
}

impl Grade {
    pub fn new(course: String, course_nickname: Option<String>, grade: f64, course_id: u32) -> Self {
        let course = if let Some(nickname) = course_nickname {
            nickname
        } else {
            course
        };

        Self {
            course,
            grade,
            course_id,
        }
    }
}

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.course, self.grade)
    }
}
