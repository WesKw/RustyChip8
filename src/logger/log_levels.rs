#[repr(i32)]
#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    None = 0,
    Light = 1,
    Standard = 2,
    Heavy = 3,
}

impl TryFrom<i32> for LogLevel {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LogLevel::None),
            1 => Ok(LogLevel::Light),
            2 => Ok(LogLevel::Standard),
            3 => Ok(LogLevel::Heavy),
            _ => Err("Invalid log level"),
        }
    }
}
