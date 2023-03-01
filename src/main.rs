mod process;
mod state;
mod todo;

use std::env;

use crate::process::process_input;
use serde_json::{Map, Value};
use state::read_file;
use todo::structs::todo_factory;

fn main() {
    // 收集所有命令行的参数，转换成数组
    let args: Vec<String> = env::args().collect();
    // 拿到第一个参数如 get、delete、edit、create
    let command = &args[1];
    // 第二个参数是用来记录事件的
    let title = &args[2];

    // 读取根目录的 state.json 文件并转换成 map json 结构
    let state: Map<String, Value> = read_file("./state.json");

    // 如果事件已经存取过了，那就直接获取，没有就将其状态设置为 pending
    let status: String;
    match state.get(title) {
        Some(result) => status = result.to_string().replace('\"', ""),
        None => status = "pending".to_string(),
    }

    // factory 工厂函数用于处理 pending 和 done 状态的输入
    let item = todo_factory(&status, title).expect(&status);
    // 将状态输入到本地文件中
    process_input(item, command.to_string(), &state);
}
