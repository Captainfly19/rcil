
use std::path::Path;

use anyhow::{self};
use clap::Parser;
use rcli::{Opts,SubCommand,process_csv};


fn main() -> anyhow::Result<()> {
    let opts:Opts = Opts::parse();
    match opts.cmd {
        SubCommand::CSV(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())  
    } else {
        Err("File does not exist")
    }
}