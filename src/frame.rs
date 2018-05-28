use variable::Variable;

pub struct Frame {
    locals: Vec<Variable>,
}

impl Frame {
    pub fn new(size: usize) -> Self {
        Frame {
            locals: Vec::with_capacity(size),
        }
    }
}
