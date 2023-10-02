use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let text_response = "{\"id\":\"1\"}";
    let resp_result = serde_json::from_str::<RdUserInfo>(&text_response);
    if let Err(e) = resp_result {
        print!("{}", e);
    }
}

#[derive(Serialize, Deserialize)]
pub struct RdUserInfo {
    pub id: i64,
}
