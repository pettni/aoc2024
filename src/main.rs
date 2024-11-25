use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "advent_of_code", version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run(RunArgs),
}

#[derive(Parser)]
struct RunArgs {
    pub day: u32,
    #[arg(short, long, default_value=None)]
    pub input: Option<PathBuf>,
}

fn main_run(args: &RunArgs) -> Result<(), Box<dyn std::error::Error>> {
    let solution = aoc2024::solutions::ALL
        .get(args.day.saturating_sub(1) as usize)
        .unwrap_or_else(|| panic!("Invalid day {}", args.day));

    let path = match &args.input {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(format!("data/{:02}.txt", args.day)),
    };
    let file_content = fs::read_to_string(path).expect("Read input");

    let out = solution.part_a(file_content.as_str());
    println!("Part a: {out}");

    let out = solution.part_b(file_content.as_str());
    println!("Part b: {out}");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match &args.command {
        Commands::Run(cmd_args) => main_run(cmd_args)?,
    }
    Ok(())
}
