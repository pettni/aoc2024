use aoc2024::{get_default_data_path, Answer};
use clap::{ArgAction, Parser, Subcommand};
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Parser)]
#[command(name = "advent_of_code", version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run(RunArgs),
    RunAll(RunAllArgs),
}

#[derive(Parser)]
struct RunArgs {
    pub day: u32,
    #[arg(short, long, default_value=None)]
    pub input: Option<PathBuf>,
    #[arg(long, short, action=ArgAction::SetTrue)]
    pub benchmark: bool,
}

#[derive(Parser)]
struct RunAllArgs {
    #[arg(long, short, action=ArgAction::SetTrue)]
    pub benchmark: bool,
}

fn part_run(f: impl Fn(&str) -> Answer, input: &str, benchmark: bool) -> (Answer, Duration, u128) {
    let t0 = Instant::now();
    let answer = {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();
        f(input)
    };
    let dt0 = t0.elapsed();

    if !benchmark {
        (answer, dt0, 1)
    } else {
        let n = (Duration::from_secs(1).as_nanos() / dt0.as_nanos().max(10)).clamp(10, 10000);
        let mut timers: Vec<Duration> = vec![];
        for _ in 0..n {
            let t0 = Instant::now();
            black_box(f(black_box(input)));
            timers.push(t0.elapsed());
        }
        let dt_nanos = timers.iter().map(|d| d.as_nanos()).sum::<u128>() / timers.len() as u128;
        let dt = Duration::from_nanos(dt_nanos as u64);
        (answer, dt, n)
    }
}

fn main_run(args: &RunArgs) -> Result<Duration, Box<dyn std::error::Error>> {
    let (part_a, part_b) = aoc2024::solutions::ALL
        .get(args.day.saturating_sub(1) as usize)
        .unwrap_or_else(|| panic!("Invalid day {}", args.day));

    let path: PathBuf = args
        .input
        .clone()
        .unwrap_or_else(|| get_default_data_path(args.day));
    let data =
        fs::read_to_string(&path).unwrap_or_else(|_| panic!("Couldn't open file {:?}", path));

    let (out_a, dt_a, n_a) = part_run(part_a, data.as_str(), args.benchmark);
    let dt_a_ms = dt_a.as_secs_f64() * 1e3;
    println!("Part a: {out_a:<16} {dt_a_ms:>10.3}ms [N={n_a}]");

    let (out_b, dt_b, n_b) = part_run(part_b, data.as_str(), args.benchmark);
    let dt_b_ms = dt_b.as_secs_f64() * 1e3;
    println!("Part b: {out_b:<16} {dt_b_ms:>10.3}ms [N={n_b}]");

    Ok(dt_a + dt_b)
}

fn main_run_all(args: &RunAllArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut total_duration: Duration = Duration::default();
    for day in 1..aoc2024::solutions::ALL.len() + 1 {
        let args = RunArgs {
            day: day as u32,
            input: None,
            benchmark: args.benchmark,
        };
        println!("Running day {day:02}");
        let day_duration = main_run(&args)?;
        total_duration += day_duration;
    }
    println!("{:=>40}", "");
    println!("Total duration: {total_duration:.3?}");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match &args.command {
        Commands::Run(cmd_args) => main_run(cmd_args).map(|_| ())?,
        Commands::RunAll(cmd_args) => main_run_all(cmd_args)?,
    }

    Ok(())
}
