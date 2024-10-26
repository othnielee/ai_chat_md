use super::error::Result;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;

pub struct TimeFormatter {
    timezone: Tz,
    format_type: TimeFormat,
}

pub enum TimeFormat {
    Rfc3339,
    Unix,
}

impl TimeFormatter {
    pub fn new(timezone: &str, format_type: TimeFormat) -> Self {
        let timezone = match timezone.parse() {
            Ok(tz) => tz,
            Err(_) => {
                println!(
                    "Warning: Invalid timezone '{}', falling back to UTC",
                    timezone
                );
                Tz::UTC
            }
        };
        Self {
            timezone,
            format_type,
        }
    }

    pub fn format_iso(&self, value: &str) -> Result<String> {
        let utc_dt = match self.format_type {
            TimeFormat::Rfc3339 => DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc),
            TimeFormat::Unix => {
                let ts = value.parse::<f64>()?;
                Utc.timestamp_opt(ts as i64, 0)
                    .single()
                    .ok_or_else(|| super::error::ParseError::InvalidTimestamp(ts))?
            }
        };

        let local_time = utc_dt.with_timezone(&self.timezone);
        Ok(format!("{}", local_time.format("%Y-%m-%d %I:%M %p %Z")))
    }

    pub fn format_unix(&self, value: f64) -> Result<String> {
        let utc_dt = Utc
            .timestamp_opt(value as i64, 0)
            .single()
            .ok_or_else(|| super::error::ParseError::InvalidTimestamp(value))?;

        let local_time = utc_dt.with_timezone(&self.timezone);
        Ok(format!("{}", local_time.format("%Y-%m-%d %I:%M %p %Z")))
    }
}
