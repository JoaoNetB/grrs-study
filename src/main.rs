use std::io::BufRead;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let file = match std::fs::File::open(&args.path) {
        Ok(file) => file,
        Err(err) => {
            return Err(err.into());
        }
    };

    let reader = std::io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => {
                return Err(err.into());
            }
        };

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
