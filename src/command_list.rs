use std::collections::HashMap;

pub struct CommandList;

impl CommandList {
    pub fn init<'a>() -> HashMap<&'a str, Vec<&'a str>> {
        let mut map = HashMap::new();
        map.insert("ls", vec!["-p"]);
        map.insert("cd", vec![]);
        map.insert("cargo", vec![]);
        map
    }
}
