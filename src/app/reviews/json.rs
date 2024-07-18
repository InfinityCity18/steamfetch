use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySummary {
    #[serde(rename = "review_score_desc")]
    pub review_score_desc: String,
    #[serde(rename = "total_positive")]
    pub total_positive: i64,
    #[serde(rename = "total_negative")]
    pub total_negative: i64,
    #[serde(rename = "total_reviews")]
    pub total_reviews: i64,
}
