mod emulator;

fn main() {
    let chip8 = emulator::Chip8::new();
    chip8.test()
}
