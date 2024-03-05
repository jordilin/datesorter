use chrono::{DateTime, Local};

pub struct CliArgs {
    pub input: String,
    pub delimiter: String,
    pub date_pos: usize,
    pub sort_mode: SortMode,
}

impl CliArgs {
    pub fn new(input: String, delimiter: String, date_pos: usize, sort_mode: SortMode) -> CliArgs {
        CliArgs {
            input,
            delimiter,
            date_pos,
            sort_mode,
        }
    }
}

pub enum SortMode {
    Asc,
    Desc,
}

pub fn sort_by_date<D: Timestamp>(mut data: Vec<D>, sort_mode: &SortMode) -> Vec<D> {
    match sort_mode {
        SortMode::Asc => data.sort_by_key(|a| a.date()),
        SortMode::Desc => data.sort_by_key(|b| std::cmp::Reverse(b.date())),
    }
    data
}

pub trait Timestamp {
    fn date(&self) -> DateTime<Local>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Data {
        date: DateTime<Local>,
    }

    impl Data {
        fn new(date: DateTime<Local>) -> Data {
            Data { date }
        }
    }

    impl Timestamp for Data {
        fn date(&self) -> DateTime<Local> {
            self.date
        }
    }

    #[test]
    fn test_sort_by_date_ascending() {
        let date1 = "2023-01-03T00:00:00Z".parse::<DateTime<Local>>().unwrap();
        let date2 = "2022-01-02T00:00:00Z".parse::<DateTime<Local>>().unwrap();
        let data = vec![Data::new(date2), Data::new(date1)];
        let expected = vec![Data::new(date2), Data::new(date1)];
        assert_eq!(sort_by_date(data, &SortMode::Asc), expected);
    }

    #[test]
    fn test_sort_by_date_descending() {
        let date1 = "2023-01-03T00:00:00Z".parse::<DateTime<Local>>().unwrap();
        let date2 = "2022-01-02T00:00:00Z".parse::<DateTime<Local>>().unwrap();
        let data = vec![Data::new(date2), Data::new(date1)];
        let expected = vec![Data::new(date1), Data::new(date2)];
        assert_eq!(sort_by_date(data, &SortMode::Desc), expected);
    }
}
