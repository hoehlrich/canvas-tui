use crate::queries::V1_URL;
use crate::types::grade::Grade;

use reqwest;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Enrollment {
    grades: Grades,
}

#[derive(Deserialize)]
struct Grades {
    current_score: Option<f64>,
}

pub async fn query_grades(course_ids: &Vec<u32>) -> Result<Vec<Grade>, Box<dyn Error>> {
    let mut responses: Vec<Grade> = vec![];
    for course_id in course_ids {
        let response = perform_query(course_id).await?;
        match response {
            Some(r) => responses.push(r),
            None => (),
        }
    }

    Ok(responses)
}

async fn perform_query(course_id: &u32) -> Result<Option<Grade>, Box<dyn Error>> {
    let api_token = std::env::var("CANVAS_API_TOKEN")?;
    let client = reqwest::Client::new();

    let grade_url = format!(
        "{}/courses/{}/enrollments?user_id=self&include[]=total_scores",
        V1_URL,
        course_id
    );
    let grade_res = client
        .get(&grade_url)
        .bearer_auth(&api_token)
        .send()
        .await?
        .json::<Vec<Enrollment>>()
        .await?;
    let grade = match grade_res.first() {
        Some(e) => e.grades.current_score,
        None => None, 
    };

    let course_url = format!(
        "{}/courses/{}",
        V1_URL,
        course_id
    );
    let course_res = client
        .get(&course_url)
        .bearer_auth(&api_token)
        .send()
        .await?
        .text()
        .await?;

    let v: serde_json::Value = serde_json::from_str(&course_res)?;
    let course = v.get("name").unwrap().as_str().unwrap().to_string();

    Ok(match grade {
        Some(g) => Some(Grade::new(course, None, g, *course_id)),
        None => None,
    })

}
