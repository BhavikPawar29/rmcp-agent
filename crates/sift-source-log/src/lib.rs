// use std::{fs, io::Write};

// use serde_json::{Value, json};
// use tokio::io::{self, AsyncBufReadExt, BufReader};



// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;



fn read_log () -> io::Result<()>{
    let file = File::open("example.txt")?;
    let mut reader = BufReader::new(file);

    // let mut line = String::new();
 
    for line in reader.lines() {
        let line = line?;
        parse_log(&line);
    }

    Ok(())
}

fn parse_log(line: &str) -> io::Result<()> {

    let log_regx = Regex::new(r"^(?P<time>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) (?P<level>[A-Z]+) \[(?P<source>[^\]]+)\] (?P<msg>.*)$"
    ).unwrap();

    for line in line.lines() {

        if let Some(caps) = log_regx.captures(&line) {
            let timestamp = &caps["time"];
            let level = &caps["level"];
            let source = &caps["source"];
            let message = &caps["msg"];

            println!("Time: {}", timestamp);
            println!("Level: {}", level);
            println!("Source: {}", source);
            println!("Message: {}", message);
        }
    }

    Ok(())

}