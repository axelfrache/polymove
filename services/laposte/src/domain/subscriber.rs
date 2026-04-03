use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub student_id: String,
    pub domain: String,
    pub channel: String,
    pub contact: String,
    pub enabled: bool,
}

impl Subscriber {
    pub fn new(student_id: String, domain: String) -> Self {
        Self {
            student_id,
            domain,
            channel: "email".to_string(),
            contact: String::new(),
            enabled: true,
        }
    }
}
