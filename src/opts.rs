use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    /// Sub-commands
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print the current time
    Time,
    /// Print the current date
    Date,
    /// Print the current time and date
    Now,
    /// Continuously print the current time and date
    Clock,
    /// Start a stopwatch.
    Stopwatch,
    /// Set a timer for a given duration.
    Timer {
        #[arg(value_enum, default_value = "00:00:10")]
        duration: Option<String>,
    },
    /// Set an alarm at a given time.
    Alarm {
        #[arg(value_enum, default_value = "12:00:00")]
        datetime: Option<String>,
    },
}
