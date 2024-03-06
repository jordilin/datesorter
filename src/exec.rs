use std::io::BufRead;

use crate::sort::{sort_by_date, CliArgs};
use crate::{input, Result};

pub fn parse_and_sort<B: BufRead>(cli_args: CliArgs, reader: B) -> Result<Vec<String>> {
    let data = input::parse(reader, &cli_args.delimiter, cli_args.date_pos)?;
    let sorted_data = sort_by_date(data, &cli_args.sort_mode);
    let sorted_data = sorted_data.into_iter().map(|p| p.line).collect();
    Ok(sorted_data)
}
