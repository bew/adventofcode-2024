use std::env;
use std::process::exit;

use anyhow::Result as AnyResult;

mod days;
use days::{Part, Day, DAYS};

fn print_usage() {
    let prog_name = env::args().next().unwrap_or("prog".to_string());
    let day_names: Vec<_> = DAYS.iter().map(|d| d.name).collect();
    println!("Usage:");
    println!("  {} <cmd>", prog_name);
    println!("  {} <day> [<custom_input_path>]", prog_name);
    println!("");

    println!("<cmd> can be:");
    println!("  all   - run all available days");
    println!("  last  - run the last available day (used while dev)");
    println!("  list  - list available days");
    println!("");

    let joined_days = day_names.join(", ");
    println!("<day> can be one of: {}", joined_days);
    println!("");
}

fn run_part(part_name: &str, part: &Part, input: &str) {
    let result = (part.func)(input);
    match (result, part.expected) {
        (None, _) => {
            println!("❌ {part_name}: Not implemented");
        }
        (Some(value), Some(expected)) => {
            if value == expected {
                println!("✅ {part_name}: {value:?} (same as expected)");
            } else {
                println!("❌ {part_name}: Expected {expected} but got {value} !!");
            }
        }
        (Some(value), None) => eprintln!("-- {part_name}: {value} ?"),
    };
}

fn run_day(day: &Day) -> AnyResult<()> {
    println!("=>> {name} - {desc}", name=day.name, desc=day.description);
    run_part("Part1", &day.part1, &day.default_input);
    run_part("Part2", &day.part2, &day.default_input);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // TODO: Use clap to parse params to structured opts!
    let prog_args: Vec<String> = env::args().collect();
    let first_arg = prog_args.get(1);

    match first_arg.map(String::as_str) {
        Some("all") => {
            for day in DAYS {
                run_day(day)?;
            }
        }
        Some("last") => {
            run_day(&DAYS.last().unwrap())?;
        }
        Some("list") => {
            println!("Available days:");
            for day in DAYS {
                println!("- {}", day.name);
                // TODO: Display how finish that day is, by running it without displaying anything
                //       (need to return proper enum with all statuses)
            }
        }
        Some(wanted_day) => {
            let matching_day = DAYS.iter().find(|day| day.name == wanted_day);
            match matching_day {
                Some(day) => run_day(day)?,
                None => {
                    println!("Unknown day '{}'", wanted_day);
                    exit(1);
                }
            };
        }
        None => {
            print_usage();
            exit(1);
        },
    };
    Ok(())
}
