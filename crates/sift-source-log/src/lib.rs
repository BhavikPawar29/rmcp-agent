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

use sift_types::Event;


fn read_log () -> io::Result<()>{
    let file = File::open("example.txt")?;
    let mut reader = BufReader::new(file);

    // let mut line = String::new();
 
    for line in reader.lines() {
        let line = line?;
        let event = parse_log(&line);

        if let Some(event)  = event {
            print!("{}", event.body);
        }
    }

    Ok(())
}

fn parse_log(line: &str) -> Option<Event> {

    let log_regx = Regex::new(r"^(?P<time>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) (?P<level>[A-Z]+) \[(?P<source>[^\]]+)\] (?P<msg>.*)$"
    ).unwrap();

    let caps = log_regx.captures(&line)?;
    
    Some (Event{ 
        timestamp: caps["time"].to_string(),
        source_id: caps["source"].to_string(),
        severity: Some(caps["level"].to_string()),
        body: caps["msg"].to_string(),
    })
    
}