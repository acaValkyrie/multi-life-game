use sdl2::pixels::Color;
use sdl2::keyboard::{Scancode, KeyboardState};
use sdl2::rect::Rect;
use std::time::Duration;

mod sdl_modules;

struct Object{
    x: u32,
    y: u32,
    color: Color,
    size: u32,
}

impl Object{
    fn new(x: u32, y: u32, color: Color, size: u32) -> Object{
        Object{x: x, y: y, color: color, size: size}
    }
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x as i32, self.y as i32, self.size, self.size)).unwrap();
    }
}

struct Cell{
    object: Object,
}

impl Cell {
    fn new(x: u32, y: u32, color: Color, size: u32) -> Cell{
        Cell{object: Object::new(x, y, color, size)}
    }
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        self.object.draw(canvas);
    }
}

pub fn main() {
    let (mut canvas, mut event_pump) 
        = sdl_modules::sdl_setup(800, 600);
    
    let mut cell = Cell::new(100, 100, Color::RGB(255, 255, 255), 100);

    // イベントループ
    'running: loop {
        if sdl_modules::is_end_event(&mut event_pump) { break 'running; }
        
        // キャンバスの初期化
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let state = &event_pump.keyboard_state();
        if state.is_scancode_pressed(Scancode::Up){    cell.object.y -= 5; }
        if state.is_scancode_pressed(Scancode::Down){  cell.object.y += 5; }
        if state.is_scancode_pressed(Scancode::Left){  cell.object.x -= 5; }
        if state.is_scancode_pressed(Scancode::Right){ cell.object.x += 5; }
        
        cell.draw(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
