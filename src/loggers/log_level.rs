pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            LogLevel::Info  => "INFO",
            LogLevel::Warn  => "WARN",
            LogLevel::Error => "Error",
        }
    }
}