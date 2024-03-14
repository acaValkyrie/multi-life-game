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
    fn set_viability(&mut self, is_alive: bool){
        if is_alive{
            self.object.color = Color::RGB(255, 255, 255);
        }else{
            self.object.color = Color::RGB(0, 0, 0);
        }
    }
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        self.object.draw(canvas);
    }
}

pub fn main() {
    let (mut canvas, mut event_pump) 
        = sdl_modules::sdl_setup(800, 600);
    
    const CELL_BOX_WIDTH_COUNT: usize = 10;
    let mut cells: [[Cell; CELL_BOX_WIDTH_COUNT]; CELL_BOX_WIDTH_COUNT];
    for i in 0..CELL_BOX_WIDTH_COUNT{
        for j in 0..CELL_BOX_WIDTH_COUNT{
            cells[i][j] = Cell::new(i as u32 * 10, j as u32 * 10, Color::RGB(0, 0, 0), 10);
            cells[i][j].set_viability(true);
        }
    }
    
    // イベントループ
    'running: loop {
        if sdl_modules::is_end_event(&mut event_pump) { break 'running; }
        
        // キャンバスの初期化
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // cell.draw(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
