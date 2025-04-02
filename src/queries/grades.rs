use crate::queries::API_URL;
use crate::types::grade::Grade;

use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",  // Update this with the correct schema path
    query_path = "src/graphql/get_grades.graphql", // Query is stored in a separate file
    response_derives = "Debug"
)]
struct GetGrades;

async fn perform_queries(course_ids: &Vec<u32>) -> Result<Vec<get_grades::ResponseData>, Box<dyn Error>> {
    let mut responses: Vec<get_grades::ResponseData> = vec![];
    for course_id in course_ids {
        let variables = get_grades::Variables { course_id: course_id.to_string() };
        let response = perform_query(variables).await?;
        responses.push(response);
    }

    Ok(responses)
}

async fn perform_query(variables: get_grades::Variables) -> Result<get_grades::ResponseData, Box<dyn Error>> {
    // this is the important line
    let request_body = GetGrades::build_query(variables);
    let api_token = std::env::var("CANVAS_API_TOKEN")?;

    let client = reqwest::Client::new();
    let res = client
        .post(API_URL)
        .bearer_auth(api_token)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<get_grades::ResponseData> = res.json().await?;

    match response_body.data {
        Some(data) => {
            Ok(data)
        }
        None => {
            Err("No data found".into())
        }
    }
}

fn parse_grades(responses: Vec<get_grades::ResponseData>) -> Result<Vec<Grade>, Box<dyn Error>> {
    let mut grades: Vec<Grade> = vec![];
    for response in responses {
        if let Some(course) = response.course {
            // Iterate over assignments
            for node in course.enrollments_connection.unwrap().nodes.unwrap() {
                if let Some(g) = node.unwrap().grades {
                    grades.push(
                        Grade::new(
                            course.name.clone(),
                            course.course_nickname.clone(),
                            match g.unposted_current_score {
                                Some(score) => score,
                                None => 100.0,
                            },
                            u32::from_str_radix(&course.id, 10)?)
                        );
                    break;
                }
            }
        }
    }

    Ok(grades)
}

pub async fn query_grades(course_ids: &Vec<u32>) -> Result<Vec<Grade>, Box<dyn Error>> {
    let responses = perform_queries(course_ids).await?;
    let grades = parse_grades(responses)?;
    Ok(grades)
}
