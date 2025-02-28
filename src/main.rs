use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod lake;
mod plugins;
mod sandbox;

/// Lake - A universal build system with Lua scripting
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom build.lake path
    #[clap(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Task to execute
    #[clap(value_name = "TASK")]
    task: Option<String>,

    /// Arguments for the task
    #[clap(value_name = "ARGS")]
    args: Vec<String>,

    /// Enable verbose logging (debug level)
    #[clap(short, long)]
    verbose: bool,
}

/// Entry point for Lake build system
fn main() {
    if let Err(err) = run() {
        log::error!("Error: {:#}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // Parse command line arguments using clap derive
    let args = Args::parse();

    // Setup the logger
    setup_logger(args.verbose);

    // Get the build.lake path
    let build_file_path = match args.file {
        Some(path) => path,
        None => lake::find_build_file().context("Could not find build.lake")?,
    };

    // Get task name and arguments
    let task_name = args.task.unwrap_or_else(|| "default".to_string());
    let task_args: Vec<&str> = args.args.iter().map(|s| s.as_str()).collect();

    // Initialize and run Lake engine with the specified build.lake and task
    lake::run_lake(&build_file_path, &task_name, &task_args)
        .context("Failed to execute Lake build system")?;

    Ok(())
}

fn setup_logger(verbose: bool) {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    if verbose {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();
}
