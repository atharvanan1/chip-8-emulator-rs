use enum_iterator::{all, Sequence};

#[derive(Debug, Clone, Copy, Sequence, PartialEq)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl From<u8> for Key {
    fn from(key: u8) -> Self {
        let key = key & 0x0F;
        match key {
            0x0 => Self::Key0,
            0x1 => Self::Key1,
            0x2 => Self::Key2,
            0x3 => Self::Key3,
            0x4 => Self::Key4,
            0x5 => Self::Key5,
            0x6 => Self::Key6,
            0x7 => Self::Key7,
            0x8 => Self::Key8,
            0x9 => Self::Key9,
            0xA => Self::KeyA,
            0xB => Self::KeyB,
            0xC => Self::KeyC,
            0xD => Self::KeyD,
            0xE => Self::KeyE,
            0xF => Self::KeyF,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for Key {
    fn into(self) -> u8 {
        match self {
            Self::Key0 => 0x0,
            Self::Key1 => 0x1,
            Self::Key2 => 0x2,
            Self::Key3 => 0x3,
            Self::Key4 => 0x4,
            Self::Key5 => 0x5,
            Self::Key6 => 0x6,
            Self::Key7 => 0x7,
            Self::Key8 => 0x8,
            Self::Key9 => 0x9,
            Self::KeyA => 0xA,
            Self::KeyB => 0xB,
            Self::KeyC => 0xC,
            Self::KeyD => 0xD,
            Self::KeyE => 0xE,
            Self::KeyF => 0xF,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyBank {
    Key0: bool,
    Key1: bool,
    Key2: bool,
    Key3: bool,
    Key4: bool,
    Key5: bool,
    Key6: bool,
    Key7: bool,
    Key8: bool,
    Key9: bool,
    KeyA: bool,
    KeyB: bool,
    KeyC: bool,
    KeyD: bool,
    KeyE: bool,
    KeyF: bool,
}

impl KeyBank {
    pub fn new() -> Self {
        Self {
            Key0: false,
            Key1: false,
            Key2: false,
            Key3: false,
            Key4: false,
            Key5: false,
            Key6: false,
            Key7: false,
            Key8: false,
            Key9: false,
            KeyA: false,
            KeyB: false,
            KeyC: false,
            KeyD: false,
            KeyE: false,
            KeyF: false,
        }
    }

    fn get_key(&self, Key: Key) -> bool {
        match Key {
            Key::Key0 => self.Key0,
            Key::Key1 => self.Key1,
            Key::Key2 => self.Key2,
            Key::Key3 => self.Key3,
            Key::Key4 => self.Key4,
            Key::Key5 => self.Key5,
            Key::Key6 => self.Key6,
            Key::Key7 => self.Key7,
            Key::Key8 => self.Key8,
            Key::Key9 => self.Key9,
            Key::KeyA => self.KeyA,
            Key::KeyB => self.KeyB,
            Key::KeyC => self.KeyC,
            Key::KeyD => self.KeyD,
            Key::KeyE => self.KeyE,
            Key::KeyF => self.KeyF,
        }
    }

    fn get_key_ref_mut(&mut self, Key: Key) -> &mut bool {
        let bank_key: &mut bool;
        match Key {
            Key::Key0 => bank_key = &mut self.Key0,
            Key::Key1 => bank_key = &mut self.Key1,
            Key::Key2 => bank_key = &mut self.Key2,
            Key::Key3 => bank_key = &mut self.Key3,
            Key::Key4 => bank_key = &mut self.Key4,
            Key::Key5 => bank_key = &mut self.Key5,
            Key::Key6 => bank_key = &mut self.Key6,
            Key::Key7 => bank_key = &mut self.Key7,
            Key::Key8 => bank_key = &mut self.Key8,
            Key::Key9 => bank_key = &mut self.Key9,
            Key::KeyA => bank_key = &mut self.KeyA,
            Key::KeyB => bank_key = &mut self.KeyB,
            Key::KeyC => bank_key = &mut self.KeyC,
            Key::KeyD => bank_key = &mut self.KeyD,
            Key::KeyE => bank_key = &mut self.KeyE,
            Key::KeyF => bank_key = &mut self.KeyF,
        }
        bank_key
    }

    pub fn get_value(&self, key: Key) -> bool {
        self.get_key(key)
    }

    pub fn set_value(&mut self, key: Key, val: bool) {
        let bank_key = self.get_key_ref_mut(key);
        *bank_key = val;
    }

    pub fn get_key_pressed(&self) -> u8 {
        for key_num in all::<Key>() {
            if self.get_key(key_num) {
                return key_num.into();
            }
        }
        0
    }
}
