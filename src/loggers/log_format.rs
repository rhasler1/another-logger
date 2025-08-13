use chrono::prelude::*;

#[derive(Clone)]
pub enum LogFormat {
    PlainText,
    JSON,
}

impl LogFormat {
    pub fn format(
        &self,
        time: &DateTime<Local>,
        level: &str,
        message: &str
    ) -> String {
        match self {
            Self::PlainText => {
                format!(
                    "{} [{}] {}\n",
                    time.format("%Y-%m-%d %H:%M:%S"),
                    level,
                    message
                )
            }
            Self::JSON => {
                format!(
                    "{{\"time\": \"{}\", \"level\": \"{}\", \"message\": \"{}\"}}\n",
                    time.format("%Y-%m-%d %H:%M:%S"),
                    level,
                    message
                )
            }
        }
    }
}