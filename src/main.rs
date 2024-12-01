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
    RunAll,
}

#[derive(Parser)]
struct RunArgs {
    pub day: u32,
    #[arg(short, long, default_value=None)]
    pub input: Option<PathBuf>,
}

fn get_default_data_path(day: u32) -> PathBuf {
    PathBuf::from(format!("data/{:02}.txt", day))
}

fn main_run(args: &RunArgs) -> Result<(), Box<dyn std::error::Error>> {
    let solution = aoc2024::solutions::ALL
        .get(args.day.saturating_sub(1) as usize)
        .unwrap_or_else(|| panic!("Invalid day {}", args.day));

    if solution.is_dummy() {
        println!("No solution implemented");
        return Ok(());
    }

    let path: PathBuf = args
        .input
        .clone()
        .unwrap_or_else(|| get_default_data_path(args.day));
    let file_content = fs::read_to_string(&path);
    if file_content.is_err() {
        println!("Could not read data at {:?}", &path);
        return Ok(());
    }

    let data = file_content.unwrap();

    let out = solution.part_a(data.as_str());
    println!("Part a: {out}");
    let out = solution.part_b(data.as_str());
    println!("Part b: {out}");

    Ok(())
}

fn main_run_all() -> Result<(), Box<dyn std::error::Error>> {
    for day in 1..26 {
        let args = RunArgs { day, input: None };
        println!("Running day {day:02}");
        main_run(&args)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match &args.command {
        Commands::Run(cmd_args) => main_run(cmd_args)?,
        Commands::RunAll => main_run_all()?,
    }

    Ok(())
}
