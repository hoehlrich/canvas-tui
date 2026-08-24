pub const GRAPHQL_URL: &str = "https://elearning.mines.edu/api/graphql";
pub const V1_URL: &str = "https://elearning.mines.edu/api/v1";

// Canvas rejects requests without a User-Agent header (403) as of 2026
pub fn client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .user_agent(concat!("canvas-tui/", env!("CARGO_PKG_VERSION")))
        .build()
}

pub mod assignments;
pub mod grades;
