mod action;
mod command;
mod display;
mod key;
mod memory;
mod reg;
mod stack;
mod timer;

use std::fmt::Display;

use command::Command;
use display::MachDisplay;
use enum_iterator::all;
use key::KeyBank;
use memory::Memory;
use reg::RegBank;
use stack::Stack;
use timer::Timer;

use self::action::Action;
use self::action::Actions;
use self::command::RawCommand;
use self::command::CommandErr;

#[derive(Debug, Clone)]
pub struct Machine {
    memory: Memory<4096>,
    display: MachDisplay<64, 32>,
    pc: u16,
    index: u16,
    stack: Stack,
    delay_timer: Timer,
    sound_timer: Timer,
    reg: RegBank,
    key: KeyBank,
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Machine {{")?;
        writeln!(f, "PC: {:#x?}", self.pc)?;
        writeln!(f, "Index: {:#x?}", self.index)?;
        writeln!(f, "{:#x?}", self.stack)?;
        writeln!(f, "{:#x?}", self.delay_timer)?;
        writeln!(f, "{:#x?}", self.sound_timer)?;
        writeln!(f, "{:#x?}", self.reg)?;
        writeln!(f, "}}")
    }
}

#[derive(Debug)]
pub struct MachineErr;

const LOAD_OFFSET: u16 = 0x200;

impl Machine {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            display: MachDisplay::new(),
            pc: LOAD_OFFSET,
            index: 0,
            stack: Stack::new(),
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            reg: RegBank::new(),
            key: KeyBank::new(),
        }
    }

    pub fn load(&mut self, prog_data: &[u8]) -> Result<(), MachineErr> {
        let mem_data = self.memory.get_mut_data(LOAD_OFFSET, prog_data.len());
        mem_data.copy_from_slice(prog_data);
        Ok(())
    }

    fn fetch_command(&mut self) -> u16 {
        let command = u16::from_be_bytes(self.memory.get_command_data(self.pc));
        self.increment_pc();
        command
    }

    fn decode_command(&self, command: u16) -> Result<Command, CommandErr> {
        RawCommand(command).try_into()
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(2);
    }

    fn decrement_pc(&mut self) {
        self.pc = self.pc.wrapping_sub(2);
    }

    fn set_pc(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn execute_command(&mut self, command: Command) -> () {
        let actions = match command {
            Command::ClearScreen => {
                self.display.clear_screen();
                Actions::new()
            }
            Command::Jump(addr) => {
                self.set_pc(addr);
                Actions::new()
            }
            Command::SkipIfRegVal(reg_x, val) => {
                if self.reg.get_value(reg_x) == val {
                    self.increment_pc();
                }
                Actions::new()
            }
            Command::SkipIfRegValNot(reg_x, val) => {
                if self.reg.get_value(reg_x) != val {
                    self.increment_pc();
                }
                Actions::new()
            }
            Command::SetVal(reg, val) => {
                self.reg.set_value(reg, val);
                Actions::new()
            }
            Command::AddVal(reg, val) => {
                self.reg.add_value(reg, val);
                Actions::new()
            }
            Command::SetIndex(val) => {
                self.index = val;
                Actions::new()
            }
            Command::Display(reg_x, reg_y, val) => {
                let actions = self.display.draw(
                    self.memory.get_data(self.index, val as usize),
                    self.reg.get_value(reg_x),
                    self.reg.get_value(reg_y),
                );
                self.display.print();
                actions
            },
            Command::SkipIfRegEqual(reg_x, reg_y) => {
                if self.reg.get_value(reg_x) == self.reg.get_value(reg_y) {
                    self.increment_pc();
                }
                Actions::new()
            }
            Command::SkipIfRegNotEqual(reg_x, reg_y) => {
                if self.reg.get_value(reg_x) != self.reg.get_value(reg_y) {
                    self.increment_pc();
                }
                Actions::new()
            }
            Command::Call(addr) => {
                self.stack.push(self.pc);
                self.set_pc(addr);
                Actions::new()
            }
            Command::Return => {
                let pc = self.stack.pop().unwrap();
                self.set_pc(pc);
                Actions::new()
            }
            Command::SetReg(reg_x, reg_y) => {
                self.reg.set_value(reg_x, self.reg.get_value(reg_y));
                Actions::new()
            }
            Command::BinOR(reg_x, reg_y) => {
                self.reg
                    .set_value(reg_x, self.reg.get_value(reg_y) | self.reg.get_value(reg_x));
                Actions::new()
            }
            Command::BinAND(reg_x, reg_y) => {
                self.reg
                    .set_value(reg_x, self.reg.get_value(reg_y) & self.reg.get_value(reg_x));
                Actions::new()
            }
            Command::LogXOR(reg_x, reg_y) => {
                self.reg
                    .set_value(reg_x, self.reg.get_value(reg_y) ^ self.reg.get_value(reg_x));
                Actions::new()
            }
            Command::AddReg(reg_x, reg_y) => {
                let val_x = self.reg.get_value(reg_x);
                let val_y = self.reg.get_value(reg_y);
                let (sum, overflow) = val_x.overflowing_add(val_y);
                if overflow {
                    self.reg.set_value(reg::Reg::VF, 1);
                }
                self.reg.set_value(reg_x, sum);
                Actions::new()
            }
            Command::SubReg(reg_x, reg_y) => {
                let val_x = self.reg.get_value(reg_x);
                let val_y = self.reg.get_value(reg_y);
                let (diff, overflow) = val_x.overflowing_sub(val_y);
                if overflow {
                    self.reg.set_value(reg::Reg::VF, 0);
                } else {
                    self.reg.set_value(reg::Reg::VF, 1);
                }
                self.reg.set_value(reg_x, diff);
                Actions::new()
            }
            Command::SubRegRev(reg_x, reg_y) => {
                let val_x = self.reg.get_value(reg_x);
                let val_y = self.reg.get_value(reg_y);
                let (diff, overflow) = val_y.overflowing_sub(val_x);
                if overflow {
                    self.reg.set_value(reg::Reg::VF, 0);
                } else {
                    self.reg.set_value(reg::Reg::VF, 1);
                }
                self.reg.set_value(reg_x, diff);
                Actions::new()
            }
            Command::ShiftLeft(reg_x, reg_y) => {
                let val_y = self.reg.get_value(reg_y);
                let (val, overflow) = val_y.overflowing_shl(1);
                if overflow {
                    self.reg.set_value(reg::Reg::VF, 1);
                }
                self.reg.set_value(reg_x, val);
                Actions::new()
            }
            Command::ShiftRight(reg_x, reg_y) => {
                let val_y = self.reg.get_value(reg_y);
                let (val, overflow) = val_y.overflowing_shr(1);
                if overflow {
                    self.reg.set_value(reg::Reg::VF, 1);
                }
                self.reg.set_value(reg_x, val);
                Actions::new()
            }
            Command::SkipIfKey(reg_x) => {
                if self
                    .key
                    .get_value(key::Key::from(self.reg.get_value(reg_x)))
                {
                    self.increment_pc();
                }
                Actions::new()
            }
            Command::SkipIfNotKey(reg_x) => {
                if !self
                    .key
                    .get_value(key::Key::from(self.reg.get_value(reg_x)))
                {
                    self.increment_pc();
                }
                Actions::new()
            }
            Command::SetRegFromDelayTimer(reg_x) => {
                self.reg.set_value(reg_x, self.delay_timer.get_value());
                Actions::new()
            }
            Command::SetDelayTimerFromReg(reg_x) => {
                self.delay_timer.set_value(self.reg.get_value(reg_x));
                Actions::new()
            }
            Command::SetSoundTimerFromReg(reg_x) => {
                self.sound_timer.set_value(self.reg.get_value(reg_x));
                Actions::new()
            }
            Command::AddIndex(reg_x) => {
                self.index = self.index.wrapping_add(self.reg.get_value(reg_x) as u16);
                if self.index & 0xF000 != 0 {
                    self.reg.set_value(reg::Reg::VF, 1);
                }
                self.index = self.index & 0x0FFF;
                Actions::new()
            }
            Command::GetKey(reg_x) => {
                self.decrement_pc();
                let key_val = self.key.get_key_pressed();
                if key_val != 0 {
                    self.reg.set_value(reg_x, key_val);
                }
                Actions::new()
            }
            Command::Font(reg_x) => {
                self.index = self.reg.get_value(reg_x) as u16;
                Actions::new()
            }
            Command::BCDConv(reg_x) => {
                let mut val_x = self.reg.get_value(reg_x) as u16;
                let mut num: u16 = 1;
                let mut digit_len = 0;
                while val_x.wrapping_div(num) != 0 {
                    num = num * 10;
                    digit_len = digit_len + 1;
                }
                digit_len = digit_len - 1;
                let data = self.memory.get_mut_data(self.index, digit_len);
                for i in digit_len - 1..=0 {
                    let val = val_x % 10;
                    data[i] = val as u8;
                    val_x = val_x / 10;
                }
                Actions::new()
            }
            Command::Store(reg_x) => {
                let index_ptr = self.memory.get_mut_data(self.index, 16);
                for reg in all::<reg::Reg>() {
                    let reg_val = self.reg.get_value(reg);
                    let reg_num = reg as u16;
                    index_ptr[reg_num as usize] = reg_val;
                    if reg_x == reg {
                        break;
                    }
                }
                Actions::new()
            }
            Command::Load(reg_x) => {
                let index_ptr = self.memory.get_data(self.index, 16);
                for reg in all::<reg::Reg>() {
                    let reg_num = reg as u16 as usize;
                    let mem_val = index_ptr[reg_num];
                    self.reg.set_value(reg, mem_val);
                    if reg_x == reg {
                        break;
                    }
                }
                Actions::new()
            }
            Command::StoreWithIndexIncrement(reg_x) => {
                let index_ptr = self.memory.get_mut_data(self.index, 16);
                for reg in all::<reg::Reg>() {
                    let reg_val = self.reg.get_value(reg);
                    let reg_num = reg as u16;
                    index_ptr[reg_num as usize] = reg_val;
                    self.index += 1;
                    if reg_x == reg {
                        break;
                    }
                }
                Actions::new()
            }
            Command::LoadWithIndexIncrement(reg_x) => {
                let index_ptr = self.memory.get_data(self.index, 16);
                for reg in all::<reg::Reg>() {
                    let reg_num = reg as u16 as usize;
                    let mem_val = index_ptr[reg_num];
                    self.reg.set_value(reg, mem_val);
                    self.index += 1;
                    if reg_x == reg {
                        break;
                    }
                }
                Actions::new()
            }
            _ => unimplemented!(),
        };

        for action in actions.into_iter() {
            match action {
                Action::SetFlag => self.reg.set_value(reg::Reg::VF, 1),
                _ => unimplemented!(),
            }
        }
    }

    pub fn step(&mut self) -> () {
        let command = self.fetch_command();
        // println!("Decoding command: {:#x?}", command);
        let command = self.decode_command(command).unwrap();
        // println!("Executing command: {:#x?}", command);
        self.execute_command(command);
    }

    pub fn run(&mut self) -> () {
        loop {
            std::thread::sleep(std::time::Duration::new(0, 16 * 1000 * 1000));
        }
    }
}
