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

use std::collections::HashMap;


pub struct Event {
    pub timestamp: String,
    pub source_id: String,
    pub severity: Option<String>,
    pub body: String,
    pub fields: HashMap<String, String>,
    pub trace_id: Option<String>,
} 