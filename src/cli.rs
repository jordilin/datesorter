use std::fmt::{Display, Formatter};

use clap::{Parser, ValueEnum};

use crate::sort::{CliArgs, SortMode};

#[derive(Parser)]
#[command(about = "Sorts data by date")]
struct Args {
    #[clap(help = "input file or - for STDIN")]
    input: String,
    #[clap(long, short, help = "delimiter", default_value = ",")]
    delimiter: String,
    #[clap(long, short, help = "date position", default_value = "0")]
    column: usize,
    #[clap(long, help = "sort mode", default_value_t=SortModeArgs::Asc)]
    sort: SortModeArgs,
}

#[derive(ValueEnum, Clone)]
enum SortModeArgs {
    Asc,
    Desc,
}

impl Display for SortModeArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortModeArgs::Asc => write!(f, "asc"),
            SortModeArgs::Desc => write!(f, "desc"),
        }
    }
}

impl From<&str> for SortModeArgs {
    fn from(s: &str) -> Self {
        match s {
            "asc" => SortModeArgs::Asc,
            "desc" => SortModeArgs::Desc,
            _ => SortModeArgs::Asc,
        }
    }
}

impl From<Args> for CliArgs {
    fn from(args: Args) -> Self {
        CliArgs::new(args.input, args.delimiter, args.column, args.sort.into())
    }
}

impl From<SortModeArgs> for SortMode {
    fn from(sort_mode: SortModeArgs) -> Self {
        match sort_mode {
            SortModeArgs::Asc => SortMode::Asc,
            SortModeArgs::Desc => SortMode::Desc,
        }
    }
}

pub fn parse_args() -> CliArgs {
    Args::parse().into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_args() {
        let args = Args::parse_from(&[
            "datesorter",
            "--delimiter",
            ";",
            "--column",
            "1",
            "--sort",
            "desc",
            "input.csv",
        ]);
        assert_eq!("input.csv", args.input);
        assert_eq!(";", args.delimiter);
        assert_eq!(1, args.column);
        assert_eq!("desc", args.sort.to_string());
    }
}
