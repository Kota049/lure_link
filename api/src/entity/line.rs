use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineToken {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub id_token: String,
}
// 最終的にはユーザーのemailとかも取得できるようにしたい
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineProfile {
    pub line_user_id: String,
}