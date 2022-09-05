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
    let chip8 = emulator::Chip8::new();

    println!("{}", args.len());
    chip8.load_rom(args[1].clone())
}
