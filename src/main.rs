use std::io::BufRead;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = std::fs::File::open(&args.path).expect("could not open the file");

    let reader = std::io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.expect("could not read the line");

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
