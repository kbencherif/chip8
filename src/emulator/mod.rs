use std::fs::File;
use std::io::Read;
pub mod graphics;
extern crate sdl2;
pub mod font;

const START_ADDRESS: i32 = 0x200;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Chip8 {
    registers: Vec<u16>,
    memory: Vec<u16>,
    i: u16,
    pc: u16,
    stack: Vec<u16>,
    sp: u8,
    screen: Vec<u16>,
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            opcode: 0,
            registers: vec![0; 16],
            memory: vec![0; 4096],
            i: 0,
            pc: 0,
            stack: vec![0; 16],
            sp: 0,
            screen: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn load_rom(&self, filename: String) {
        let mut f = File::open(&filename).expect("Can't open rom");
        let mut buf = vec![0, f.metadata().expect("Can't read metadata").len() as u8];

        f.read_to_end(&mut buf).expect("Can't read rom");
    }

    pub fn load_font(&mut self) {
        for i in 0..font::FONTSET_SIZE {
            self.memory[font::FONTSET_START_ADDRESS as usize + i] = font::FONT[i]
        }
    }

    pub fn run(&self) {
        let mut screen = graphics::Screen::new(WIDTH as u32, HEIGHT as u32);
        let mut i = 0;
        loop {
            screen.clear(i);
            i = (i + 1) % 255;
            screen.draw();
            if screen.poll_event() {
                break;
            }
        }
    }

    pub fn _debug(&self) {
        println!("{:?}", self)
    }
}
