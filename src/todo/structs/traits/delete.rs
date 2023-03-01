use serde_json::{Map, Value};

use crate::state::{clean_file, write_file};

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        if state.is_empty() {
            clean_file("./state.json")
        } else {
            write_file("./state.json", state);
        }
        println!("\n\n{} is being deleted\n\n", title);
    }
}
