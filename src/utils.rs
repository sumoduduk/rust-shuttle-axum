use chrono::{DateTime, Utc};

pub fn string_to_datetime(input: &str) -> eyre::Result<DateTime<Utc>> {
    let date = DateTime::parse_from_rfc2822(input)?.with_timezone(&Utc);

    Ok(date)
}

pub fn parse_date(date_str: &str) -> eyre::Result<i64> {
    let dt = DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z")?;
    Ok(dt.timestamp())
}
