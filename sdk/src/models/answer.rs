/*
 * Swagger relearn
 *
 * relearn
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: m@m.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Answer {
    #[serde(rename = "questionId")]
    pub question_id: i64,
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "org_id", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
}

impl Answer {
    pub fn new(question_id: i64) -> Answer {
        Answer {
            question_id,
            valid: None,
            created_at: None,
            user_id: None,
            org_id: None,
        }
    }
}