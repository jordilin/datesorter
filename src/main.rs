use datesorter::{cli, exec, input};

fn main() {
    let args = cli::parse_args();
    let buf_reader = input::read_file(&args.input).unwrap();
    let sorted_data = exec::parse_and_sort(args, buf_reader).unwrap();
    for line in sorted_data {
        println!("{}", line);
    }
}
