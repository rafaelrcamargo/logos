pub mod github;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct BasicResponse {
    pub code: String,
    pub state: String
}
