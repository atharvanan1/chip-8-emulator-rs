#[derive(Debug, Clone, Copy)]
pub struct Memory<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> Memory<N> {
    pub fn new() -> Self {
        Self { data: [0; N] }
    }

    pub fn get_data(&self, offset: u16, size: usize) -> &[u8] {
        let offset = offset as usize;
        let size = size;
        if (offset > N) || (offset + size > N) {
            panic!("Out of bounds access");
        }
        &self.data[offset..offset + size]
    }

    pub fn get_mut_data(&mut self, offset: u16, size: usize) -> &mut [u8] {
        let offset = offset as usize;
        let size = size;
        if (offset > N) || (offset + size > N) {
            panic!("Out of bounds access");
        }
        &mut self.data[offset..offset + size]
    }

    pub fn get_command_data(&self, pc: u16) -> [u8; 2] {
        let command_data = self.get_data(pc, 2);
        if command_data.len() != 2 {
            panic!("Invalid data length");
        }
        [command_data[0], command_data[1]]
    }
}
