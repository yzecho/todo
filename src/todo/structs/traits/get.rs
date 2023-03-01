use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {} \n\n", result);
            }
            None => println!("item: {} was not find", title),
        }
    }
}
