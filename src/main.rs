use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let matches = grrs::find_matches(&args.path, &args.pattern).with_context(|| {
        format!(
            "Error searching for `{}` in {}",
            &args.pattern,
            &args.path.display()
        )
    })?;

    let mut stdout = io::BufWriter::new(io::stdout());
    for line in matches {
        writeln!(stdout, "{}", line)?;
    }

    return Ok(());
}
