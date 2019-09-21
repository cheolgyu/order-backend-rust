// general response msg struct

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Msg {
    pub status: i32,
    pub data: serde_json::Value,
}
