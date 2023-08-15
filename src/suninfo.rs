use crate::config::Config;
use crate::error::{AppError, Error};

use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, TimeZone, Utc};
use sunrise_sunset_calculator::SunriseSunsetParameters;

#[derive(Debug)]
pub struct SunInfo {
    pub now: DateTime<FixedOffset>,
    pub sunrise: DateTime<FixedOffset>,
    pub sunset: DateTime<FixedOffset>,
}

impl TryFrom<&Config> for SunInfo {
    type Error = Error;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let hour = 3600;

        let timezone = FixedOffset::east_opt(config.clock.timezone * hour).ok_or(AppError::TimezoneConstruct)?;
        let now = Utc::now().with_timezone(&timezone);
        log::debug!("current datetime: {now}");

        // we need to know todays sunrise and sunset
        let midday = timezone
            .with_ymd_and_hms(now.year(), now.month(), now.day(), 12, 0, 0)
            .single()
            .ok_or(AppError::MiddayConstruct)?
            .timestamp();

        let result = SunriseSunsetParameters::new(midday, config.coord.latitude, config.coord.longitude).calculate()?;
        log::debug!("{result:?}");

        let from_timestamp = |timestamp, timezone| {
            NaiveDateTime::from_timestamp_opt(timestamp, 0)
                .as_ref()
                .map(|dt| Utc.from_utc_datetime(dt))
                .ok_or(AppError::TimestampParse)
                .map(|dt| dt.with_timezone(timezone))
        };

        let sunrise = from_timestamp(result.rise, &timezone)?;
        let sunset = from_timestamp(result.set, &timezone)?;

        Ok(SunInfo { now, sunrise, sunset })
    }
}
