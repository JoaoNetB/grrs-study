use anyhow::{Context, Result};
use std::io::BufRead;
use std::io::{self, Write};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let sdtout = io::stdout();
    let mut handle = io::BufWriter::new(sdtout);

    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    let reader = std::io::BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) => {
                return Err(err.into());
            }
        };

        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }

    Ok(())
}
