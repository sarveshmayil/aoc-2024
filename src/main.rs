use advent_of_code::template::commands::{solve, time};
use args::{parse, AppArguments};

mod args {
    use advent_of_code::template::Day;
    use std::process;

    pub enum AppArguments {
        Solve {
            day: Day,
            release: bool,
            dhat: bool,
        },
        Time {
            day: Day,
        },
    }

    pub fn parse() -> Result<AppArguments, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("solve") => AppArguments::Solve {
                day: args.opt_free_from_str()?.ok_or("Missing day argument")?,
                release: args.contains("--release"),
                dhat: args.contains("--dhat"),
            },
            Some("time") => AppArguments::Time{
                day: args.opt_free_from_str()?.ok_or("Missing day argument")?,
            },
            Some(x) => {
                eprintln!("Unknown command: {}", x);
                process::exit(1);
            }
            None => {
                eprintln!("No command provided");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Unknown arguments: {:?}", remaining);
            process::exit(1);
        }

        Ok(app_args)
    }
}

fn main() {
    match parse() {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArguments::Solve { day, release, dhat } => solve::handle(day, release, dhat),
            AppArguments::Time { day } => time::handle(day),
        }
    }
}