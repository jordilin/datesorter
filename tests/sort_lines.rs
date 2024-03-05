use datesorter::{
    exec,
    sort::{CliArgs, SortMode},
};

const LINES_ISO8601: &str = r#"Lorem ipsum dolor sit amet, 2023-09-05T08:20:15Z, consectetur adipiscing elit
Fusce euismod justo nec, 2024-05-18T14:45:30Z, ultricies semper
Nulla facilisi, 2022-12-10T11:10:25Z, aliquam euismod"#;

#[test]
fn test_sort_lines_in_iso8601_fmt_desc() {
    let cli_args = CliArgs::new("input.csv".to_string(), ",".to_string(), 1, SortMode::Desc);
    let reader = LINES_ISO8601.as_bytes();
    let sorted_data = exec::parse_and_sort(cli_args, reader).unwrap();
    assert_eq!(
        sorted_data,
        vec![
            "Fusce euismod justo nec,2024-05-18T14:45:30Z,ultricies semper",
            "Lorem ipsum dolor sit amet,2023-09-05T08:20:15Z,consectetur adipiscing elit",
            "Nulla facilisi,2022-12-10T11:10:25Z,aliquam euismod",
        ]
    );
}

#[test]
fn test_sort_lines_in_iso8601_fmt_asc() {
    let cli_args = CliArgs::new("input.csv".to_string(), ",".to_string(), 1, SortMode::Asc);
    let reader = LINES_ISO8601.as_bytes();
    let sorted_data = exec::parse_and_sort(cli_args, reader).unwrap();
    assert_eq!(
        sorted_data,
        vec![
            "Nulla facilisi,2022-12-10T11:10:25Z,aliquam euismod",
            "Lorem ipsum dolor sit amet,2023-09-05T08:20:15Z,consectetur adipiscing elit",
            "Fusce euismod justo nec,2024-05-18T14:45:30Z,ultricies semper",
        ]
    );
}
