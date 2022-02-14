use std::path::PathBuf;

use fltk::{
    app, button, dialog,
    enums::{Color, FrameType},
    frame,
    prelude::*,
    text, window,
};
use fltk_flex::{Flex, FlexType};
const MEMORY_SIZE: usize = 4096;
const REGISTER_NUM: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DELAY_TIMER_TICK: u8 = 60;
type Inst = u16;

#[allow(dead_code)]
struct Chip8Mach {
    memory: [u8; MEMORY_SIZE],
    stack: Vec<usize>,
    registers: [u8; REGISTER_NUM],
    pc: usize,
    sp: usize,
    ip: usize,
    display_buffer: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
}

#[allow(dead_code)]
impl Chip8Mach {
    fn new() -> Chip8Mach {
        Chip8Mach {
            memory: [0; MEMORY_SIZE],
            stack: Vec::new(),
            registers: [0; REGISTER_NUM],
            pc: 0,
            sp: 0,
            ip: 0,
            display_buffer: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
        }
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = window::Window::new(100, 100, 680, 600, "Chip 8 Emulator");
    let mut screen_frame = frame::Frame::default().with_size(640, 320).with_pos(20, 20);
    screen_frame.set_frame(FrameType::EngravedBox);
    screen_frame.set_color(Color::Black);

    let mut menu_frame = frame::Frame::default()
        .with_size(680, 280)
        .below_of(&screen_frame, 20);
    let mut btn = button::Button::default()
        .with_size(80, 40)
        .with_label("Start")
        .below_of(&screen_frame, 20)
        .center_x(&screen_frame);
    btn.set_callback(move |b| {
        b.set_label("Stop");
        b.redraw();
        // Call start_emulation function here
    });
    btn.set_color(Color::Red);

    let mut filename = PathBuf::default();
    let mut open_btn = button::Button::default()
        .with_size(80, 40)
        .with_label("Open ROM")
        .with_pos(btn.x(), btn.y() + 40);
    open_btn.set_callback(move |b| {
        let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
        dlg.set_option(dialog::FileDialogOptions::NoOptions);
        dlg.set_filter("*.{rom,chip8rom,hex}");
        dlg.show();
        let get_filename = dlg.filename();
        let mut buf = text::TextBuffer::default();
        if !get_filename.to_string_lossy().to_string().is_empty() {
            if get_filename.exists() {
                match buf.load_file(&get_filename) {
                    Ok(_) => filename = get_filename,
                    Err(e) => dialog::alert(
                        0,
                        0,
                        &format!("An issue occured while loading the file: {}", e),
                    ),
                }
            } else {
                dialog::alert(0, 0, "File does not exist!")
            }
        }
    });

    wind.make_resizable(false);
    wind.end();
    wind.show();

    app.run().unwrap();
}
