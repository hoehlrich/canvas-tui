use crate::queries::GRAPHQL_URL;
use crate::types::assignment::Assignment;

use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

// Define the GraphQL query
type URL = String;
type DateTime = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",  // Update this with the correct schema path
    query_path = "src/graphql/get_assignments.graphql", // Query is stored in a separate file
    response_derives = "Debug"
)]
struct GetAssignments;

async fn perform_queries(
    course_ids: &Vec<u32>,
) -> Result<Vec<get_assignments::ResponseData>, Box<dyn Error>> {
    let mut responses: Vec<get_assignments::ResponseData> = vec![];
    for course_id in course_ids {
        let variables = get_assignments::Variables {
            course_id: course_id.to_string(),
        };
        let response = perform_query(variables).await?;
        responses.push(response);
    }

    Ok(responses)
}

async fn perform_query(
    variables: get_assignments::Variables,
) -> Result<get_assignments::ResponseData, Box<dyn Error>> {
    // this is the important line
    let request_body = GetAssignments::build_query(variables);
    let api_token = std::env::var("CANVAS_API_TOKEN")?;

    let client = reqwest::Client::new();
    let res = client
        .post(GRAPHQL_URL)
        .bearer_auth(api_token)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<get_assignments::ResponseData> = res.json().await?;

    match response_body.data {
        Some(data) => Ok(data),
        None => Err("No data found".into()),
    }
}

fn parse_assignments(
    responses: Vec<get_assignments::ResponseData>,
) -> Result<Vec<Assignment>, Box<dyn Error>> {
    let mut assignments: Vec<Assignment> = vec![];
    let now = chrono::Utc::now();
    for response in responses {
        if let Some(course) = response.course {
            // Iterate over assignments
            for a in course.assignments_connection.unwrap().nodes.unwrap() {
                let a = a.unwrap();
                let completed = a.submissions_connection.unwrap().nodes.unwrap().len() != 0;
                let assignment: Assignment = Assignment::new(
                    a.name.clone().unwrap(),
                    course.course_nickname.clone(),
                    a.description.clone(),
                    a.html_url.clone().unwrap(),
                    a.due_at,
                    course.name.clone(),
                    completed,
                    a.lock_info.unwrap().is_locked,
                )?;
                if let Some(due) = assignment.date {
                    // If assignment due within 14 days, add to list
                    if due > now && due < now + chrono::Duration::days(21) {
                        assignments.push(assignment);
                    }
                }
            }
        } else {
            return Err("Assignment parsing failed".into());
        }
    }

    return Ok(assignments);
}

fn sort_assignments(assignments: &mut Vec<Assignment>) {
    assignments.sort_by(|a, b| a.date.unwrap().cmp(&b.date.unwrap()));
}

pub async fn query_assignments(course_ids: &Vec<u32>) -> Result<Vec<Assignment>, Box<dyn Error>> {
    let responses = perform_queries(course_ids).await?;
    let mut assignments = parse_assignments(responses)?;
    sort_assignments(&mut assignments);
    Ok(assignments)
}
