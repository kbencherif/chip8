use std::env;
mod emulator;

fn print_help() {
    println!("Usage: cargo run rom")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_help();
        return
    }
    let mut chip8 = emulator::Chip8::new();

    chip8.load_rom(args[1].clone());
    chip8.load_font();
    //chip8._debug();
    chip8.run();
}
