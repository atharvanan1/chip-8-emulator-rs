use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Sequence)]
#[repr(u16)]
pub enum Reg {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl From<u8> for Reg {
    fn from(reg: u8) -> Self {
        let reg = reg & 0x0F;
        match reg {
            0x0 => Self::V0,
            0x1 => Self::V1,
            0x2 => Self::V2,
            0x3 => Self::V3,
            0x4 => Self::V4,
            0x5 => Self::V5,
            0x6 => Self::V6,
            0x7 => Self::V7,
            0x8 => Self::V8,
            0x9 => Self::V9,
            0xA => Self::VA,
            0xB => Self::VB,
            0xC => Self::VC,
            0xD => Self::VD,
            0xE => Self::VE,
            0xF => Self::VF,
            _ => unreachable!(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct RegBank {
    V0: u8,
    V1: u8,
    V2: u8,
    V3: u8,
    V4: u8,
    V5: u8,
    V6: u8,
    V7: u8,
    V8: u8,
    V9: u8,
    VA: u8,
    VB: u8,
    VC: u8,
    VD: u8,
    VE: u8,
    VF: u8,
}

impl RegBank {
    pub fn new() -> Self {
        Self {
            V0: 0,
            V1: 0,
            V2: 0,
            V3: 0,
            V4: 0,
            V5: 0,
            V6: 0,
            V7: 0,
            V8: 0,
            V9: 0,
            VA: 0,
            VB: 0,
            VC: 0,
            VD: 0,
            VE: 0,
            VF: 0,
        }
    }

    fn get_reg_ref(&self, reg: Reg) -> &u8 {
        let bank_reg: &u8;
        match reg {
            Reg::V0 => bank_reg = &self.V0,
            Reg::V1 => bank_reg = &self.V1,
            Reg::V2 => bank_reg = &self.V2,
            Reg::V3 => bank_reg = &self.V3,
            Reg::V4 => bank_reg = &self.V4,
            Reg::V5 => bank_reg = &self.V5,
            Reg::V6 => bank_reg = &self.V6,
            Reg::V7 => bank_reg = &self.V7,
            Reg::V8 => bank_reg = &self.V8,
            Reg::V9 => bank_reg = &self.V9,
            Reg::VA => bank_reg = &self.VA,
            Reg::VB => bank_reg = &self.VB,
            Reg::VC => bank_reg = &self.VC,
            Reg::VD => bank_reg = &self.VD,
            Reg::VE => bank_reg = &self.VE,
            Reg::VF => bank_reg = &self.VF,
        }
        bank_reg
    }

    fn get_reg_ref_mut(&mut self, reg: Reg) -> &mut u8 {
        let bank_reg: &mut u8;
        match reg {
            Reg::V0 => bank_reg = &mut self.V0,
            Reg::V1 => bank_reg = &mut self.V1,
            Reg::V2 => bank_reg = &mut self.V2,
            Reg::V3 => bank_reg = &mut self.V3,
            Reg::V4 => bank_reg = &mut self.V4,
            Reg::V5 => bank_reg = &mut self.V5,
            Reg::V6 => bank_reg = &mut self.V6,
            Reg::V7 => bank_reg = &mut self.V7,
            Reg::V8 => bank_reg = &mut self.V8,
            Reg::V9 => bank_reg = &mut self.V9,
            Reg::VA => bank_reg = &mut self.VA,
            Reg::VB => bank_reg = &mut self.VB,
            Reg::VC => bank_reg = &mut self.VC,
            Reg::VD => bank_reg = &mut self.VD,
            Reg::VE => bank_reg = &mut self.VE,
            Reg::VF => bank_reg = &mut self.VF,
        }
        bank_reg
    }

    pub fn get_value(&self, reg: Reg) -> u8 {
        let bank_reg = self.get_reg_ref(reg);
        *bank_reg
    }

    pub fn set_value(&mut self, reg: Reg, val: u8) {
        let bank_reg = self.get_reg_ref_mut(reg);
        *bank_reg = val;
    }

    pub fn add_value(&mut self, reg: Reg, val: u8) {
        let bank_reg = self.get_reg_ref_mut(reg);
        *bank_reg = bank_reg.wrapping_add(val);
    }
}
