use std::fs;
use std::io::{Write, copy};
use std::str;
use reqwest::Client;
use serde::*;
use serde_json::*;


#[derive(Deserialize, Debug)]
struct Headers{
    content_len: String,
}

#[tokio::main]
pub async fn download(args: Vec<String>) -> Result<(), reqwest::Error> {
   
    // Creates a file

    let mut file = fs::File::create(args[0].clone()).expect("Error occured while creating file");

    // Writes that file with the data stored in the data variable

    file.write_all(args[1]).expect("Error while downloading file");

    Ok(())
}