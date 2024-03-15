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
    
    const CELL_BOX_WIDTH_COUNT: usize = 10;
    let mut cell_viabilities: [[bool; CELL_BOX_WIDTH_COUNT]; CELL_BOX_WIDTH_COUNT] = [[false; CELL_BOX_WIDTH_COUNT]; CELL_BOX_WIDTH_COUNT];
    for i in 0..CELL_BOX_WIDTH_COUNT{
        for j in 0..CELL_BOX_WIDTH_COUNT{
            cell_viabilities[i][j] = true;
        }
    }
    
    // イベントループ
    'running: loop {
        if sdl_modules::is_end_event(&mut event_pump) { break 'running; }
        
        // キャンバスの初期化
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        for i in 0..CELL_BOX_WIDTH_COUNT{
            for j in 0..CELL_BOX_WIDTH_COUNT{
                if cell_viabilities[i][j] == true{
                    let cell = Cell::new((i * 50) as u32, (j * 50) as u32, Color::RGB(255, 255, 255), 50);
                    cell.draw(&mut canvas);
                }
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
