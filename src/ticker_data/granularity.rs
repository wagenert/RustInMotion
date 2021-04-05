use crate::prelude::*;
#[derive(Debug, Clone, PartialEq)]
pub enum Granularity {
    Minute,
    Day,
}

impl Granularity {
    pub fn to_string(self: &Self) -> &str {
        match self {
            Granularity::Day => "1d",
            Granularity::Minute => "1m",
        }
    }

    #[allow(dead_code)]
    pub fn get_minimum(from_date: DateTime<Utc>) -> Granularity {
        let dur = Utc::now() - from_date;
        if dur.num_days() < 7 {
            Granularity::Minute
        } else {
            Granularity::Day
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_granularity_minute() {
        let today = Utc::today().and_hms(0, 0, 0);
        assert_eq!(Granularity::get_minimum(today), Granularity::Minute);
    }

    #[test]
    fn test_granularity_minute_latest() {
        let last_week = Utc::now() - Duration::days(6);
        assert_eq!(Granularity::get_minimum(last_week), Granularity::Minute);
    }

    #[test]
    fn test_granularity_days() {
        let last_week = Utc::today().and_hms(0, 0, 0) - Duration::days(8);
        assert_eq!(Granularity::get_minimum(last_week), Granularity::Day);
    }

    #[test]
    fn test_string_day() {
        let granularity = Granularity::Day;
        assert_eq!(Granularity::to_string(&granularity), "1d");
    }

    #[test]
    fn test_string_minute() {
        let granularity = Granularity::Minute;
        assert_eq!(Granularity::to_string(&granularity), "1m");
    }
}
