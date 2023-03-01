use serde_json::{Map, Value};

use crate::todo::structs::{
    done::Done,
    pending::Pending,
    traits::{create::Create, delete::Delete, edit::Edit, get::Get},
    ItemTypes,
};

// 处理 pending 状态
fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    // 根据用户的输入来调用不同的方法
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(
            &item.super_struct.title,
            &item.super_struct.status,
            &mut state,
        ),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command: {} is not supported", command),
    }
}

// 处理 done 状态
fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} is not supported", command),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
