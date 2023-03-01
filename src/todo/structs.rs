mod base;
pub mod done;
pub mod pending;
pub mod traits;

use done::Done;
use pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

// 状态处理工厂函数
pub fn todo_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    match item_type {
        "pending" => {
            // 创建一个 pending 状态的数据结构，然后返回出去
            let pending_item = Pending::new(item_title);
            Ok(ItemTypes::Pending(pending_item))
        }
        "done" => {
            // 创建一个 done 状态的数据结构，然后返回出去
            let done_item = Done::new(item_title);
            Ok(ItemTypes::Done(done_item))
        }
        // 如果不是这两个状态就返回一个错误
        _ => Err("This is not accepted!"),
    }
}
