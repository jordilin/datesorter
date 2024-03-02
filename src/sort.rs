use chrono::{DateTime, Local};

pub enum SortMode {
    Asc,
    Desc,
}

pub fn sort_by_date<D: Timestamp>(data: Vec<D>, sort_mode: &SortMode) -> Vec<D> {
    let mut data = data;
    match sort_mode {
        SortMode::Asc => data.sort_by(|a, b| a.date().cmp(&b.date())),
        SortMode::Desc => data.sort_by(|a, b| b.date().cmp(&a.date())),
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
