extern crate sdl2;

use std::time::Duration;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump, Sdl,
};

pub struct Screen {
    canvas: Canvas<Window>,
    e: EventPump,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
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

        let window = match window_result {
            Ok(it) => it,
            Err(e) => panic!("Window error {}", e),
        };

        let canvas_result = window.into_canvas().build();
        let canvas = match canvas_result {
            Ok(it) => it,
            Err(e) => panic!("Canvas error {}", e),
        };

        let event_pump_result = sdl_context.event_pump();
        let e = match event_pump_result {
            Ok(it) => it,
            Err(e) => panic!("Event_pump error {}", e),
        };

        Screen { canvas, e }
    }

    pub fn clear(&mut self, i: u8) {
        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        self.canvas.clear();
    }

    pub fn draw(&mut self) {
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn poll_event(&mut self) -> bool {
        for event in self.e.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }
        false
    }
}
