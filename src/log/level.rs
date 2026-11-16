use std::fmt::Display;

pub enum LogLevel {
    INFO,
    WARN,
    DEBUG,
    ERROR,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            LogLevel::INFO => "INFO".to_string(),
            LogLevel::WARN => "WARN".to_string(),
            LogLevel::DEBUG => "DEBUG".to_string(),
            LogLevel::ERROR => "ERROR".to_string(),
        };
        write!(f, "{}", str)
    }
}
