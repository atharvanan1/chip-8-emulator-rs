use super::reg::Reg;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    ExecuteMachineRoutine(u16),
    ClearScreen,
    Jump(u16),
    Call(u16),
    Return,
    Skip,
    SkipIfRegVal(Reg, u8),
    SkipIfRegValNot(Reg, u8),
    SkipIfRegEqual(Reg, Reg),
    SkipIfRegNotEqual(Reg, Reg),
    SetVal(Reg, u8),
    AddVal(Reg, u8),
    SetReg(Reg, Reg),
    BinOR(Reg, Reg),
    BinAND(Reg, Reg),
    LogXOR(Reg, Reg),
    AddReg(Reg, Reg),
    SubReg(Reg, Reg),
    SubRegRev(Reg, Reg),
    ShiftLeft(Reg, Reg),
    ShiftRight(Reg, Reg),
    SetIndex(u16),
    JumpWithOffset(u16, Reg),
    Random(Reg, u8),
    Display(Reg, Reg, u8),
    SkipIfKey(Reg),
    SkipIfNotKey(Reg),
    SetRegFromDelayTimer(Reg),
    SetDelayTimerFromReg(Reg),
    SetSoundTimerFromReg(Reg),
    AddIndex(Reg),
    GetKey(Reg),
    Font(Reg),
    BCDConv(Reg),
    Load(Reg),
    Store(Reg),
    LoadWithIndexIncrement(Reg),
    StoreWithIndexIncrement(Reg),
}

#[derive(Debug, Clone, Copy)]
pub struct CommandConstruct {
    command: u16,
    reg_x: Reg,
    reg_y: Reg,
    val_u4: u8,
    val_u8: u8,
    val_u12: u16,
}

impl CommandConstruct {
    pub fn new(command: u16) -> Self {
        let reg_x = Reg::from(((command >> 8) & 0x000F) as u8);
        let reg_y = Reg::from(((command >> 4) & 0x000F) as u8);
        let val_u4: u8 = (command & 0x000F) as u8;
        let val_u8: u8 = (command & 0x00FF) as u8;
        let val_u12: u16 = command & 0x0FFF;
        Self {
            command,
            reg_x,
            reg_y,
            val_u4,
            val_u8,
            val_u12,
        }
    }

    fn decode(&self) -> Result<Command, CommandErr> {
        let command = self.command;
        match command {
            0x00E0 => Ok(Command::ClearScreen),
            0x00EE => Ok(Command::Return),
            command if (command & 0xF000) == 0x1000 => Ok(Command::Jump(self.val_u12)),
            command if (command & 0xF000) == 0x2000 => Ok(Command::Call(self.val_u12)),
            command if (command & 0xF000) == 0x3000 => {
                Ok(Command::SkipIfRegVal(self.reg_x, self.val_u8))
            }
            command if (command & 0xF000) == 0x4000 => {
                Ok(Command::SkipIfRegValNot(self.reg_x, self.val_u8))
            }
            command if (command & 0xF000) == 0x5000 => {
                Ok(Command::SkipIfRegEqual(self.reg_x, self.reg_y))
            }
            command if (command & 0xF000) == 0x6000 => Ok(Command::SetVal(self.reg_x, self.val_u8)),
            command if (command & 0xF000) == 0x7000 => Ok(Command::AddVal(self.reg_x, self.val_u8)),
            command if (command & 0xF00F) == 0x8000 => Ok(Command::SetReg(self.reg_x, self.reg_y)),
            command if (command & 0xF00F) == 0x8001 => Ok(Command::BinOR(self.reg_x, self.reg_y)),
            command if (command & 0xF00F) == 0x8002 => Ok(Command::BinAND(self.reg_x, self.reg_y)),
            command if (command & 0xF00F) == 0x8003 => Ok(Command::LogXOR(self.reg_x, self.reg_y)),
            command if (command & 0xF00F) == 0x8004 => Ok(Command::AddReg(self.reg_x, self.reg_y)),
            command if (command & 0xF00F) == 0x8005 => Ok(Command::SubReg(self.reg_x, self.reg_y)),
            command if (command & 0xF00F) == 0x8006 => {
                Ok(Command::ShiftRight(self.reg_x, self.reg_y))
            }
            command if (command & 0xF00F) == 0x8007 => {
                Ok(Command::SubRegRev(self.reg_x, self.reg_y))
            }
            command if (command & 0xF00F) == 0x800E => {
                Ok(Command::ShiftLeft(self.reg_x, self.reg_y))
            }
            command if (command & 0xF000) == 0x9000 => {
                Ok(Command::SkipIfRegNotEqual(self.reg_x, self.reg_y))
            }
            command if (command & 0xF000) == 0xA000 => Ok(Command::SetIndex(self.val_u12)),
            command if (command & 0xF000) == 0xD000 => {
                Ok(Command::Display(self.reg_x, self.reg_y, self.val_u4))
            }
            command if (command & 0xF0FF) == 0xE09E => Ok(Command::SkipIfKey(self.reg_x)),
            command if (command & 0xF0FF) == 0xE0A1 => Ok(Command::SkipIfNotKey(self.reg_x)),
            command if (command & 0xF0FF) == 0xF007 => {
                Ok(Command::SetRegFromDelayTimer(self.reg_x))
            }
            command if (command & 0xF0FF) == 0xF015 => {
                Ok(Command::SetDelayTimerFromReg(self.reg_x))
            }
            command if (command & 0xF0FF) == 0xF018 => {
                Ok(Command::SetSoundTimerFromReg(self.reg_x))
            }
            command if (command & 0xF0FF) == 0xF01E => Ok(Command::AddIndex(self.reg_x)),
            command if (command & 0xF0FF) == 0xF00A => Ok(Command::GetKey(self.reg_x)),
            command if (command & 0xF0FF) == 0xF029 => Ok(Command::Font(self.reg_x)),
            command if (command & 0xF0FF) == 0xF033 => Ok(Command::BCDConv(self.reg_x)),
            command if (command & 0xF0FF) == 0xF055 => Ok(Command::Store(self.reg_x)),
            command if (command & 0xF0FF) == 0xF065 => Ok(Command::Load(self.reg_x)),
            _ => Err(CommandErr),
        }
    }
}

#[derive(Debug)]
pub struct CommandErr;

impl TryInto<Command> for CommandConstruct {
    type Error = CommandErr;

    fn try_into(self) -> Result<Command, Self::Error> {
        self.decode()
    }
}
