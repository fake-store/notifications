use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderShippedMessage {
    pub order_id: String,
    pub user_id: String,
    pub tracking_number: String,
    pub trace_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
}
