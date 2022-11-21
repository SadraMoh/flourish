use druid::{Data, Lens};

#[derive(Clone, Data, Lens, Default)]
pub struct TaskState {
    pub description: String,
    pub is_complete: bool,
}

impl TaskState {
    pub fn new() -> Self {
        TaskState {
            description: "New Task".to_string(),
            is_complete: false,
        }
    }
}
