use anyhow::Context;

use clap::{Parser, ValueEnum};

use tokio::sync::OnceCell;

use tracing::debug;

/// Execute commands in parallel
///
/// By Aaron Riekenberg <aaron.riekenberg@gmail.com>
///
/// https://github.com/aaronriekenberg/rust-parallel
/// https://crates.io/crates/rust-parallel
#[derive(Parser, Debug, Default)]
#[command(verbatim_doc_comment, version)]
pub struct CommandLineArgs {
    /// Run commands from arguments only.
    #[arg(short, long)]
    pub commands_from_args: bool,

    /// Discard output for commands
    #[arg(short, long)]
    pub discard_output: Option<DiscardOutput>,

    /// Input file or - for stdin.  Defaults to stdin if no inputs are specified.
    #[arg(short, long)]
    pub input: Vec<String>,

    /// Maximum number of commands to run in parallel, defauts to num cpus
    #[arg(short, long, default_value_t = num_cpus::get(), value_parser = parse_semaphore_permits)]
    pub jobs: usize,

    /// Use null separator for reading input instead of newline.
    #[arg(short('0'), long)]
    pub null_separator: bool,

    /// Use shell for running commands.
    ///
    /// If $SHELL environment variable is set use it else use /bin/bash.
    ///
    /// Each input line is passed to $SHELL -c <line> as a single argument.
    #[arg(short, long)]
    pub shell: bool,

    /// Output channel capacity
    #[arg(short, long, default_value_t = 1, value_parser = parse_semaphore_permits)]
    pub output_channel_capacity: usize,

    /// Optional command and initial arguments to run for each input line.
    #[arg(trailing_var_arg(true))]
    pub command_and_initial_arguments: Vec<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum DiscardOutput {
    /// Redirect stdout for commands to /dev/null
    Stdout,
    /// Redirect stderr for commands to /dev/null
    Stderr,
    /// Redirect stdout and stderr for commands to /dev/null
    All,
}

fn parse_semaphore_permits(s: &str) -> Result<usize, String> {
    let range = 1..=tokio::sync::Semaphore::MAX_PERMITS;

    let value: usize = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
    if range.contains(&value) {
        Ok(value)
    } else {
        Err(format!("value not in range {:?}", range))
    }
}

static INSTANCE: OnceCell<CommandLineArgs> = OnceCell::const_new();

pub fn initialize() -> anyhow::Result<()> {
    let command_line_args = CommandLineArgs::parse();

    debug!("command_line_args = {:?}", command_line_args);

    INSTANCE
        .set(command_line_args)
        .context("INSTANCE.set error")?;

    Ok(())
}

pub fn instance() -> &'static CommandLineArgs {
    INSTANCE.get().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clap_configuation() {
        use clap::CommandFactory;

        CommandLineArgs::command().debug_assert()
    }
}
