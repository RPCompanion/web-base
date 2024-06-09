
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericAPIError {
    pub message: String
}

impl GenericAPIError {

    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string()
        }
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

}