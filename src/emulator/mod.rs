pub struct Chip8 {
    opcode: u16,
    memory: Vec<u16>,
    i: u16,
    pc: u16,
    stack: Vec<u16>,
    sp: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            opcode: 0,
            memory: vec![0, 4096],
            i: 0,
            pc: 0,
            stack: vec![0, 16],
            sp: 0,
        }
    }

    pub fn test(&self) {
        println!("Flex")
    }
}
