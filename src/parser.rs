use crate::{command_line_args::CommandLineArgs, common::OwnedCommandAndArgs};

pub mod buffered;
pub mod command_line;

fn build_shell_command_and_args(command_line_args: &CommandLineArgs) -> Option<Vec<String>> {
    if command_line_args.shell {
        Some(vec![command_line_args.shell_path.clone(), "-c".to_owned()])
    } else {
        None
    }
}

fn prepend_shell_command_and_args(
    shell_command_and_args: &[String],
    command_and_args: Vec<String>,
) -> Option<OwnedCommandAndArgs> {
    let mut result = Vec::with_capacity(shell_command_and_args.len() + 1);
    result.extend_from_slice(shell_command_and_args);
    result.push(command_and_args.join(" "));

    OwnedCommandAndArgs::try_from(result).ok()
}
