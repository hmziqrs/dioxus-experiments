#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StateStatus {
    Init,
    Loading,
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateSlice<T: Clone> {
    pub status: StateStatus,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Clone> StateSlice<T> {
    pub fn new() -> Self {
        Self {
            status: StateStatus::Init,
            data: None,
            message: None,
        }
    }

    pub fn is_init(&self) -> bool {
        self.status == StateStatus::Init
    }

    pub fn is_loading(&self) -> bool {
        self.status == StateStatus::Loading
    }

    pub fn is_success(&self) -> bool {
        self.status == StateStatus::Success
    }

    pub fn is_failed(&self) -> bool {
        self.status == StateStatus::Failed
    }
}
