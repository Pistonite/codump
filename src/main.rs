//! Entry point for the executable

use clap::Parser;

/// Cli entrypoint
fn main() {
    if let Err(e) = main_internal() {
        eprintln!("error: {e}");
    }
}

/// Internal entrypoint that returns result
fn main_internal() -> Result<(), String> {
    let args = codump::CliArgs::parse();
    let file = args.file.clone();
    let search_path = args.search_path.clone();
    let config = args.try_into()?;
    let output = codump::execute(&file, &search_path, &config)?;

    for line in output {
        println!("{line}");
    }

    Ok(())
}
