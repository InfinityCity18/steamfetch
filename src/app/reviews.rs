mod json;

pub use json::QuerySummary;

use super::links::APP_REVIEWS;
use crate::error::{ExitResult, IntoResultExitError};
use serde_json::Value;

impl QuerySummary {
    pub fn get_app_reviews(app_id: u32) -> ExitResult<'static, QuerySummary> {
        let url = APP_REVIEWS.replace("{}", &app_id.to_string());
        let response = reqwest::blocking::get(url).into_exit_error("fetching reviews failed")?;

        let mut reviews: Value = response
            .json()
            .into_exit_error("parsing reviews json failed")?;

        let query_sum: QuerySummary = serde_json::from_value(reviews["query_summary"].take())
            .into_exit_error("parsing reviews json failed")?;
        Ok(query_sum)
    }
}
