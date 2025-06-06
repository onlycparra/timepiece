mod clock;
mod common;
mod format;
mod opts;
mod parse;
mod print;
mod stopwatch;
mod timer;

use clap::Parser;
use opts::Command;
fn main() {
    let args = opts::Opts::parse();
    let command = match args.command {
        Some(cmd) => cmd,
        None => Command::Now,
    };

    match command {
        Command::Time => clock::time(),
        Command::Date => clock::date(),
        Command::Now => clock::now(),
        Command::Clock => clock::clock(),
        Command::Stopwatch => {
            if atty::is(atty::Stream::Stdin) {
                stopwatch::stopwatch();
            } else {
                stopwatch::stopwatch_notatty();
            }
        }
        Command::Timer { duration } => {
            let duration = duration.unwrap();
            let dur = parse::dur(&duration);
            match dur {
                Some(d) => timer::timer(d),
                None => eprintln!("Error: incorrect timer duration"),
            };
        }
        Command::Alarm { datetime } => {
            let datetime = datetime.unwrap();
            let dtime = parse::time(&datetime);
            match dtime {
                Some(dt) => timer::alarm(dt),
                None => eprintln!("Error: incorrect alarm time"),
            };
        }
    }
}
