#[derive(Debug, Clone)]
pub struct Stack {
    data: Vec<u16>,
}

#[derive(Debug)]
pub struct StackErr;

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::<u16>::new(),
        }
    }

    pub fn push(&mut self, addr: u16) {
        self.data.push(addr);
    }

    pub fn pop(&mut self) -> Result<u16, StackErr> {
        let addr = self.data.pop();
        match addr {
            Some(addr) => Ok(addr),
            None => Err(StackErr),
        }
    }
}
