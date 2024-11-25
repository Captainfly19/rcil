// use anyhow;
mod csv;
mod genpass;
mod base64;
mod text;

use std::path::Path;

use self::{csv::CsvOpts,genpass::GenPassOpts};
use clap::Parser;


pub use self::{csv::OutputFormat,
    base64::{Base64SubCommand,Base64Format},
    text::{TextSignFormat,TextSubCommand},
};


#[derive(Debug,Parser)]
#[command(name = "rcli",version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd:SubCommand,
}

#[derive(Debug,Parser)]


pub enum SubCommand{
    #[command(name = "csv",about = "Show CSV ,or convert CSV to other formats")]
    CSV(CsvOpts),
    #[command(name = "genpass",about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
}//这是接口.终端输入指令的时候 -- 后面跟的东西

fn verify_file(filename: &str) -> Result<String, &'static str> {
    //if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists(){
        Ok(filename.into())
    }else {
        Err("File does not exist")
    }
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_verify_input_file(){
        assert_eq!(verify_file("-"),Ok("-".into()));
        assert_eq!(verify_file("*"),Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"),Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"),Err("File does not exist"));
    }
}