use druid::{
    im::{vector, Vector},
    Data, Lens,
};

use crate::components::task::TaskState;

#[derive(Clone, Data, Lens, Default)]
pub struct ScrollState {
    pub name: String,
    pub tasks: Vector<TaskState>,

    pub is_editing_name: bool,    
}

impl ScrollState {
    pub fn new() -> Self {
        ScrollState {
            name: "New Scroll".to_string(),
            tasks: vector![TaskState::new(), TaskState::new(), TaskState::new(),],
            is_editing_name: false,
        }
    }
}
