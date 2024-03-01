use std::io::BufRead;

use crate::Result;
use chrono::{DateTime, Local};

pub struct PreSort {
    pub date: DateTime<Local>,
    pub fields: Vec<String>,
    pub date_pos: usize,
}

impl PreSort {
    fn new(date: DateTime<Local>, fields: Vec<String>, date_pos: usize) -> PreSort {
        PreSort {
            date,
            fields,
            date_pos,
        }
    }
}

pub fn parse_input<R: BufRead>(input: R, delimiter: &str, date_pos: usize) -> Result<Vec<PreSort>> {
    let data = input
        .lines()
        .map(|line| {
            let line = line?;
            let fields: Vec<String> = line
                .split(delimiter)
                .map(|s| s.trim().to_string())
                .collect();
            let date = fields[date_pos].parse::<DateTime<Local>>()?;
            Ok(PreSort::new(date, fields, date_pos))
        })
        .collect::<Result<Vec<PreSort>>>()?;
    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let line1 = "first_field, second_field, 2021-01-01T00:00:00Z\n";
        let line2 = "second_line_first_field, second_line_second_field, 2022-01-02T00:00:00Z\n";
        let date1 = "2021-01-01T00:00:00Z".to_string();
        let date2 = "2022-01-02T00:00:00Z".to_string();
        let expected = vec![
            PreSort::new(
                date1.parse::<DateTime<Local>>().unwrap(),
                vec!["first_field".to_string(), "second_field".to_string(), date1],
                2,
            ),
            PreSort::new(
                date2.parse::<DateTime<Local>>().unwrap(),
                vec![
                    "second_line_first_field".to_string(),
                    "second_line_second_field".to_string(),
                    date2,
                ],
                2,
            ),
        ];
        let input = format!("{}{}", line1, line2);
        let result = parse_input(input.as_bytes(), ",", 2).unwrap();
        assert_eq!(expected[0].date, result[0].date);
        assert_eq!(expected[0].fields, result[0].fields);
        assert_eq!(expected[0].date_pos, result[0].date_pos);
        assert_eq!(expected[1].date, result[1].date);
        assert_eq!(expected[1].fields, result[1].fields);
        assert_eq!(expected[1].date_pos, result[1].date_pos);
    }
}
