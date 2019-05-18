// general response msg struct

#[derive(Deserialize, Serialize, Debug)]
pub struct Msg {
    pub status: i32,
    pub data: serde_json::Value,
}
