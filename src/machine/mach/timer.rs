#[derive(Debug, Clone, Copy)]
pub struct Timer {
    val: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self { val: 0 }
    }

    #[allow(dead_code)]
    pub fn increment(&mut self) {
        self.val = self.val.wrapping_add(1);
    }

    #[allow(dead_code)]
    pub fn decrement(&mut self) {
        self.val = self.val.wrapping_sub(1);
    }

    pub fn set_value(&mut self, val: u8) {
        self.val = val;
    }

    pub fn get_value(&self) -> u8 {
        self.val
    }
}
