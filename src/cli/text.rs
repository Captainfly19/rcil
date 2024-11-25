use core::{fmt, str};
use std::{default, str::FromStr};

use clap::Parser;

use super::verify_file;

#[derive(Debug,Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
}

#[derive(Debug,Parser)]
pub struct TextSignOpts{
    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub input:String,
    #[arg(short,long,value_parser = verify_file)]
    pub key:String,
    #[arg(long,default_value = "black3" ,value_parser = parse_format)]
    pub format:TextSignFormat,
}

#[derive(Debug,Parser)]
pub struct TextVerifyOpts{
    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub input:String,
    #[arg(short,long,value_parser = verify_file,default_value = "-")]
    pub key:String,
    #[arg(short,long)]
    pub sig:String,
    #[arg(long,default_value = "black3" ,value_parser = parse_format)]
    pub format:TextSignFormat,
}

#[derive(Debug,Clone,Copy)]
pub enum TextSignFormat {
    Black3,
    Ed25519,
}

fn parse_format(format:&str) -> Result<TextSignFormat,anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    
    fn from_str(s:&str) -> Result<Self,Self::Err> {
        match s{
            "black3" => Ok(TextSignFormat::Black3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Black3 => "black3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self , f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",Into::<&str>::into(*self))
    }
}