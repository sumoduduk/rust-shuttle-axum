use chrono::DateTime;

pub fn parse_date(date_str: &str) -> eyre::Result<i64> {
    let dt = DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z")?;
    Ok(dt.timestamp())
}

