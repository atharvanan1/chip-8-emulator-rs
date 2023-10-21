use super::reg::Reg;

#[allow(dead_code)]
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

#[derive(Clone, Copy, Debug)]
pub struct RawCommand(pub u16);

impl RawCommand {
    fn reg_x(self) -> Reg {
        let nibble = (self.0 >> 8) as u8 & 0x0F;
        Reg::from(nibble)
    }

    fn reg_y(self) -> Reg {
        let nibble = (self.0 >> 4) as u8 & 0x0F;
        Reg::from(nibble)
    }

    fn val4(self) -> u8 {
        self.0 as u8 & 0x0F
    }

    fn val8(self) -> u8 {
        self.0 as u8 & 0xFF
    }

    fn val12(self) -> u16 {
        self.0 & 0x0FFF
    }

    fn decode(&self) -> Result<Command, CommandErr> {
        let command = self.0;
        match command {
            0x00E0 => Ok(Command::ClearScreen),
            0x00EE => Ok(Command::Return),
            _ => match command >> 12 {
                0x1 => Ok(Command::Jump(self.val12())),
                0x2 => Ok(Command::Call(self.val12())),
                0x3 => Ok(Command::SkipIfRegVal(self.reg_x(), self.val8())),
                0x4 => Ok(Command::SkipIfRegValNot(self.reg_x(), self.val8())),
                0x5 => Ok(Command::SkipIfRegEqual(self.reg_x(), self.reg_y())),
                0x6 => Ok(Command::SetVal(self.reg_x(), self.val8())),
                0x7 => Ok(Command::AddVal(self.reg_x(), self.val8())),
                0x8 => match command & 0x000F {
                    0x0 => Ok(Command::SetReg(self.reg_x(), self.reg_y())),
                    0x1 => Ok(Command::BinOR(self.reg_x(), self.reg_y())),
                    0x2 => Ok(Command::BinAND(self.reg_x(), self.reg_y())),
                    0x3 => Ok(Command::LogXOR(self.reg_x(), self.reg_y())),
                    0x4 => Ok(Command::AddReg(self.reg_x(), self.reg_y())),
                    0x5 => Ok(Command::SubReg(self.reg_x(), self.reg_y())),
                    0x6 => Ok(Command::ShiftRight(self.reg_x(), self.reg_y())),
                    0x7 => Ok(Command::SubRegRev(self.reg_x(), self.reg_y())),
                    0xE => Ok(Command::ShiftLeft(self.reg_x(), self.reg_y())),
                    _ => Err(CommandErr)
                },
                0x9 => Ok(Command::SkipIfRegNotEqual(self.reg_x(), self.reg_y())),
                0xA => Ok(Command::SetIndex(self.val12())),
                0xD => Ok(Command::Display(self.reg_x(), self.reg_y(), self.val4())),
                0xE => match command & 0x00FF {
                    0x9E => Ok(Command::SkipIfKey(self.reg_x())),
                    0xA1 => Ok(Command::SkipIfNotKey(self.reg_x())),
                    _ => Err(CommandErr),
                }
                0xF => match command & 0x00FF {
                    0x07 => Ok(Command::SetRegFromDelayTimer(self.reg_x())),
                    0x0A => Ok(Command::GetKey(self.reg_x())),
                    0x15 => Ok(Command::SetDelayTimerFromReg(self.reg_x())),
                    0x18 => Ok(Command::SetSoundTimerFromReg(self.reg_x())),
                    0x1E => Ok(Command::AddIndex(self.reg_x())),
                    0x29 => Ok(Command::Font(self.reg_x())),
                    0x33 => Ok(Command::BCDConv(self.reg_x())),
                    0x55 => Ok(Command::Store(self.reg_x())),
                    0x65 => Ok(Command::Load(self.reg_x())),
                    _ => Err(CommandErr),
                }
                _ => Err(CommandErr),
            },
        }
    }
}

#[derive(Debug)]
pub struct CommandErr;

impl TryInto<Command> for RawCommand {
    type Error = CommandErr;

    fn try_into(self) -> Result<Command, Self::Error> {
        self.decode()
    }
}
