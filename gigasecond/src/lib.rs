use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let one_gigasecond = 1000000000;
    start + Duration::seconds(one_gigasecond)
}
