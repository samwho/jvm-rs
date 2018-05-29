use frame::Frame;

pub struct Stack {
    stack: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
}
