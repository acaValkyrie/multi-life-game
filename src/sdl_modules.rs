use sdl2::render::Canvas;
use sdl2::EventPump;

pub fn sdl_setup(window_width: u32, window_height: u32) -> (Canvas<sdl2::video::Window>, EventPump){
    // SDL2の初期化
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // ウィンドウの作成
    let window = video_subsystem
        .window("Rust-SDL2", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    // キャンバスの作成
    let canvas = window.into_canvas().build().unwrap();

    // イベントループの作成
    let event_pump = sdl_context.event_pump().unwrap();

    (canvas, event_pump)
}

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn is_end_event(event_pump: &mut sdl2::EventPump) -> bool{
    for event in event_pump.poll_iter() {
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