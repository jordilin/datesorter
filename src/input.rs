use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{sort::Timestamp, Result};
use chrono::{DateTime, Local};

pub struct PreSort {
    pub date: DateTime<Local>,
    pub line: String,
    pub date_pos: usize,
}

impl PreSort {
    fn new(date: DateTime<Local>, fields: String, date_pos: usize) -> PreSort {
        PreSort {
            date,
            line: fields,
            date_pos,
        }
    }
}

impl Timestamp for PreSort {
    fn date(&self) -> DateTime<Local> {
        self.date
    }
}

pub fn parse<R: BufRead>(input: R, delimiter: &str, date_pos: usize) -> Result<Vec<PreSort>> {
    let data = input
        .lines()
        .map(|line| {
            let line = line?;
            let fields: Vec<String> = line
                .split(delimiter)
                .map(|s| s.trim().to_string())
                .collect();
            let date = fields[date_pos].parse::<DateTime<Local>>()?;
            Ok(PreSort::new(date, line, date_pos))
        })
        .collect::<Result<Vec<PreSort>>>()?;
    Ok(data)
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<BufReader<fs::File>> {
    let file = std::fs::File::open(path)?;
    Ok(std::io::BufReader::new(file))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let line1 = "first_field, second_field, 2021-01-01T00:00:00Z";
        let line2 = "second_line_first_field, second_line_second_field, 2022-01-02T00:00:00Z";
        let date1 = "2021-01-01T00:00:00Z".to_string();
        let date2 = "2022-01-02T00:00:00Z".to_string();
        let expected = vec![
            PreSort::new(
                date1.parse::<DateTime<Local>>().unwrap(),
                line1.to_string(),
                2,
            ),
            PreSort::new(
                date2.parse::<DateTime<Local>>().unwrap(),
                line2.to_string(),
                2,
            ),
        ];
        let input = format!("{}\n{}\n", line1, line2);
        let result = parse(input.as_bytes(), ",", 2).unwrap();
        assert_eq!(expected[0].date, result[0].date);
        assert_eq!(expected[0].line, result[0].line);
        assert_eq!(expected[0].date_pos, result[0].date_pos);
        assert_eq!(expected[1].date, result[1].date);
        assert_eq!(expected[1].line, result[1].line);
        assert_eq!(expected[1].date_pos, result[1].date_pos);
    }
}
