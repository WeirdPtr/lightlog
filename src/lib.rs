use chrono;
use colored::Colorize;

#[derive(Debug)]
pub enum LoggingLevel {
    Full = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

#[derive(Debug)]
pub enum LoggingType {
    None = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Debug = -1,
}

#[derive(Debug)]
pub struct Logger {
    log_level: LoggingLevel,
    default_origin: String,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            log_level: LoggingLevel::Full,
            default_origin: "".to_string(),
        }
    }
}

impl Logger {
    pub fn new(log_level: LoggingLevel, default_origin: String) -> Self {
        Self {
            log_level,
            default_origin,
        }
    }

    pub fn set_level(&mut self, level: LoggingLevel) {
        use LoggingLevel::*;

        match level {
            Full => {
                self.log_level = Full;
            }
            Info => {
                self.log_level = Info;
            }
            Warning => {
                self.log_level = Warning;
            }
            Error => {
                self.log_level = Error;
            }
        }
    }

    pub fn set_default_origin<T>(&mut self, default_origin: T)
    where
        T: Into<String>,
    {
        self.default_origin = default_origin.into();
    }

    fn get_log_msg<T>(&self, log_prefix: &str, msg: T, log_origin: String) -> String
    where
        T: Into<String>,
    {
        let timestamp = chrono::Utc::now();

        let date = timestamp.date().to_string().replace("UTC", "");
        let time = timestamp.time().format("%H:%M:%S").to_string();

        let colored_log_prefix = match log_prefix {
            "INFO" => log_prefix.green(),
            "ERROR" => log_prefix.bright_red(),
            "WARNING" => log_prefix.yellow(),
            "DEBUG" => log_prefix.blue(),
            _ => log_prefix.normal(),
        }
        .to_string();

        let mut origin: String;

        if !log_origin.is_empty() {
            origin = " [".to_string();
            origin.push_str(&log_origin.bright_white().to_string());
            origin.push_str("]");
        } else {
            origin = log_origin;
        }

        let log_msg = format!(
            "[{date}{time}] [{prefix}]{log_origin}: {message}",
            date = date,
            time = time,
            prefix = colored_log_prefix,
            log_origin = origin,
            message = msg.into()
        );

        return log_msg;
    }

    fn log_info<T>(&self, msg: T, log_origin: String)
    where
        T: Into<String>,
    {
        use LoggingLevel::*;

        let msg = self.get_log_msg("INFO", msg, log_origin);

        match self.log_level {
            Full | Info => {
                println!("{}", msg);
            }
            _ => return,
        };
    }

    fn log_warning<T>(&self, msg: T, log_origin: String)
    where
        T: Into<String>,
    {
        use LoggingLevel::*;

        let msg = self.get_log_msg("WARNING", msg, log_origin);

        match self.log_level {
            Full | Info | Warning => {
                println!("{}", msg);
            }
            _ => return,
        };
    }

    fn log_error<T>(&self, msg: T, log_origin: String)
    where
        T: Into<String>,
    {
        let msg = self.get_log_msg("ERROR", msg, log_origin);

        println!("{}", msg);
    }

    fn log_debug<T>(&self, msg: T, log_origin: String)
    where
        T: Into<String>,
    {
        use LoggingLevel::*;

        let msg = self.get_log_msg("DEBUG", msg, log_origin);

        match self.log_level {
            Full => {
                println!("{}", msg);
            }
            _ => return,
        };
    }

    pub fn log_origin_message<T>(
        &self,
        message: T,
        message_log_type: LoggingType,
        log_origin: Option<T>,
    ) where
        T: Into<String>,
    {
        let origin = match log_origin {
            Some(_) => log_origin.unwrap().into(),
            None => self.default_origin.clone(),
        };

        match message_log_type {
            LoggingType::Info => self.log_info(message, origin),
            LoggingType::Warning => self.log_warning(message, origin),
            LoggingType::Error => self.log_error(message, origin),
            LoggingType::Debug => self.log_debug(message, origin),
            LoggingType::None => return,
        }
    }

    pub fn log_message<T>(&self, message: T, message_log_type: LoggingType)
    where
        T: Into<String>,
    {
        self.log_origin_message(message, message_log_type, None);
    }
}