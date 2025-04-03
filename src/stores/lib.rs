#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StateFrameStatus {
    Init,
    Loading,
    Success,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateFrame<T: Clone> {
    pub status: StateFrameStatus,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Clone> StateFrame<T> {
    pub fn new() -> Self {
        Self {
            status: StateFrameStatus::Init,
            data: None,
            message: None,
        }
    }

    pub fn is_init(&self) -> bool {
        self.status == StateFrameStatus::Init
    }

    pub fn is_loading(&self) -> bool {
        self.status == StateFrameStatus::Loading
    }

    pub fn is_success(&self) -> bool {
        self.status == StateFrameStatus::Success
    }

    pub fn is_failed(&self) -> bool {
        self.status == StateFrameStatus::Failed
    }

    pub fn set_loading(&mut self, message: Option<String>) {
        self.status = StateFrameStatus::Loading;
        self.message = message;
    }

    pub fn set_success(&mut self, data: Option<T>, message: Option<String>) {
        self.status = StateFrameStatus::Success;
        self.data = data;
        self.message = message;
    }

    pub fn set_failed(&mut self, message: Option<String>) {
        self.status = StateFrameStatus::Failed;
        self.message = message;
    }
}
