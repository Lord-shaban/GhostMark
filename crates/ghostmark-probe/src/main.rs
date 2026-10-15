//! Read-only diagnostic entry point. This binary has no system-changing commands.

use std::{
    env,
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let arguments: Vec<_> = env::args_os().skip(1).collect();
    if arguments.len() == 1 && arguments[0] == "--help" {
        println!(
            "Usage: ghostmark-probe inspect\n\nPrint a privacy-limited JSON environment report. Does not hide or restore a watermark.\nInspection success (exit 0) does not mean the environment is supported."
        );
        return ExitCode::SUCCESS;
    }
    if arguments.len() != 1 || arguments[0] != "inspect" {
        eprintln!("Usage: ghostmark-probe inspect\nOnly read-only inspection is implemented.");
        return ExitCode::from(2);
    }
    let stdout = io::stdout();
    let mut output = stdout.lock();
    if serde_json::to_writer_pretty(&mut output, &ghostmark_core::inspect()).is_err()
        || writeln!(output).is_err()
    {
        eprintln!("Could not write the inspection report.");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
