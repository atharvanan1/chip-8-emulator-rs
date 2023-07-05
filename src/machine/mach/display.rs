use super::action::Action;
use super::action::Actions;
use super::reg::Reg;

#[derive(Debug, Clone, Copy)]
pub struct Display<const X: usize, const Y: usize> {
    data: [[bool; X]; Y],
}

#[derive(Debug)]
pub struct DisplayErr;

impl<const X: usize, const Y: usize> Display<X, Y> {
    pub fn new() -> Self {
        Self {
            data: [[false; X]; Y],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<&bool, DisplayErr> {
        if x > X || y > Y {
            return Err(DisplayErr);
        }
        Ok(&self.data[y][x])
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Result<&mut bool, DisplayErr> {
        if x > X || y > Y {
            return Err(DisplayErr);
        }
        Ok(&mut self.data[y][x])
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, val: bool) -> Result<(), DisplayErr> {
        let pixel = self.get_pixel_mut(x, y)?;
        *pixel = val;
        Ok(())
    }

    pub fn clear_screen(&mut self) {
        for x in 0..X {
            for y in 0..Y {
                let _ = self.set_pixel(x, y, false);
            }
        }
    }

    fn u8_to_bools_le(val: u8) -> [bool; 8] {
        let mut bools = [false; 8];
        bools[0] = val & 0b1000_0000 != 0;
        bools[1] = val & 0b0100_0000 != 0;
        bools[2] = val & 0b0010_0000 != 0;
        bools[3] = val & 0b0001_0000 != 0;
        bools[4] = val & 0b0000_1000 != 0;
        bools[5] = val & 0b0000_0100 != 0;
        bools[6] = val & 0b0000_0010 != 0;
        bools[7] = val & 0b0000_0001 != 0;
        bools
    }

    pub fn draw(&mut self, sprite_data: &[u8], x: u8, y: u8) -> Actions {
        let mut x = x as usize % X;
        let mut y = y as usize % Y;
        let mut actions = Actions::new();
        'outer: for byte in sprite_data.into_iter() {
            for bit in Self::u8_to_bools_le(*byte).into_iter() {
                if bit && *self.get_pixel(x, y).unwrap() {
                    self.set_pixel(x, y, false).unwrap();
                    actions.push(Action::SetFlag);
                } else if bit && *self.get_pixel(x, y).unwrap() == false {
                    self.set_pixel(x, y, bit).unwrap();
                }
                x = x + 1;
                if x == X {
                    y = y + 1;
                    if y == Y {
                        break 'outer;
                    }
                }
                x = x as usize % X;
            }
        }
        actions
    }
}
