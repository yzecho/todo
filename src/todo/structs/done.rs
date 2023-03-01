use super::base::Base;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        Done {
            super_struct: Base::new(input_title, "done"),
        }
    }
}

impl Delete for Done {}
impl Edit for Done {}
impl Get for Done {}
