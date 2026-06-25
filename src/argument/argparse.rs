// src/argument/argparse.rs

// For Devs:
// the randomizer_timer default var is now at src/behavior/traits.rs

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct ArgParser {
    /// Allows quiet execution, detatches from terminal, persists running.
    #[arg(short, long)]
    pub quiet: bool,

    /// Allows random Trigger run.
    #[arg(short, long)]
    pub randomizer: bool,

    /// Randomizer Trigger in seconds. For every 300 seconds (default) of random timer interval, BSOD may trigger or not
    #[arg(short = 'c', long)]
    pub randomizer_timer: Option<u64>,

    /// Chances of BSOD in a random event (1 - 100%, 10 - 10%)
    #[arg(short = 'x' , long, default_value = "10")]
    pub randomizer_freq: u64,

    /// Time in Seconds to BSOD.
    #[arg(short, long, default_value = "1200")]
    pub timeout: u64,

    /// Erase Dump memory if found
    #[arg(short = 'd', long, default_value = "false")]
    pub erase_dump: bool,

    /// Show possible error codes
    #[arg(long)]
    pub possible_codes: bool,

    /// Set BSOD Error
    #[arg(short = 'l', long, default_value = "00")]
    pub bsod_code: String,

    /// Suprise me, random BSOD counter
    #[arg(short = 'y', long)]
    pub suprise_me: bool,

}
