use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum HealthCheck {
    Ok,
    Warning,
    Critical,
    Unknown,
}

impl Default for HealthCheck {
    fn default() -> HealthCheck { HealthCheck::Unknown }
}

impl From<i8> for HealthCheck {
    fn from(value: i8) -> HealthCheck {
        match value {
            0 => HealthCheck::Ok,
            1 => HealthCheck::Warning,
            2 => HealthCheck::Critical,
            3 => HealthCheck::Unknown,
            _ => HealthCheck::Unknown,
        }
    }
}

impl fmt::Display for HealthCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            HealthCheck::Ok => "OK",
            HealthCheck::Warning => "WARNING",
            HealthCheck::Critical => "CRITICAL",
            HealthCheck::Unknown => "UNKNOWN",
        };
        write!(f, "{}", msg)
    }
}
