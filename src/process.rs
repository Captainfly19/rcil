use csv::Reader;
use serde::{Serialize,Deserialize};
use std::process::Output;
use std::{fs,io};
use anyhow;

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]

struct Player {
    // #[serde(rename = "Name")]
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
    
}

pub fn process_csv(input:&str,output:&str) -> Result<(),io::Error> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    } 
    let json = serde_json::to_string_pretty(&ret)?;//生成一个string版本的json
    fs::write(output, json)?; // => ()
    Ok(())// => Result
    
}