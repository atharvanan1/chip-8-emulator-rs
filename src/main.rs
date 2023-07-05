use chip8emu::machine::Machine;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct Emulation {
    reader: BufReader<File>,
    mach: Machine,
}

impl Emulation {
    pub fn new(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        Self {
            reader: reader,
            mach: Machine::new(),
        }
    }

    pub fn start_emulation(&mut self) -> std::io::Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        self.reader.read_to_end(&mut buf)?;
        self.mach.load(&buf).unwrap();
        loop {
            self.mach.step();
        }
    }
}

fn main() {
    let file = PathBuf::from("test_opcode.ch8");
    let mut emulation = Emulation::new(file.as_path());
    emulation.start_emulation().unwrap();
}
