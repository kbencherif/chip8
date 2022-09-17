extern crate sdl2;

use sdl2::video::Window;

pub fn setup(width: u32, height: u32) -> Window {
    let sdl_context_result = sdl2::init();
    let sdl_context = match sdl_context_result {
        Ok(it) => it,
        Err(e) => panic!("Something went wrong with sdl: {}", e),
    };
    let video_subsystem_result = sdl_context.video();
    let video_subsystem = match video_subsystem_result {
        Ok(it) => it,
        Err(e) => panic!("Something went wrong with sdl: {}", e),
    };

    let window_result = video_subsystem
        .window("chip8", width, height)
        .position_centered()
        .build();

    match window_result {
        Ok(it) => it,
        Err(e) => panic!("Window error {}", e),
    }
}
