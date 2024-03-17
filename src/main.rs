use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use rand::Rng;

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
    fn set_position(&mut self, x: u32, y: u32){
        self.object.x = x;
        self.object.y = y;
    }
}

pub fn main() {
    
    let cell_size: u32 = 4;
    let color_green = Color::RGB(133, 219, 109);
    let color_blue = Color::RGB(86, 212, 187);
    let color_yellow = Color::RGB(220, 234, 127);
    
    const CELL_BOX_WIDTH_COUNT: usize = 200;
    const CELL_BOX_HEIGHT_COUNT: usize = 200;
    const CELL_TYPE_COUNT: usize = 3;
    let mut cell: [Cell; CELL_TYPE_COUNT] = [Cell::new(0, 0, color_green, cell_size), Cell::new(0, 0, color_blue, cell_size), Cell::new(0, 0, color_yellow, cell_size)];
    let mut cell_viabilities_before: [[[bool; CELL_TYPE_COUNT]; CELL_BOX_WIDTH_COUNT]; CELL_BOX_HEIGHT_COUNT] = [[[false; CELL_TYPE_COUNT]; CELL_BOX_WIDTH_COUNT]; CELL_BOX_HEIGHT_COUNT];
    let mut cell_viabilities_after: [[[bool; CELL_TYPE_COUNT]; CELL_BOX_WIDTH_COUNT]; CELL_BOX_HEIGHT_COUNT] = [[[false; CELL_TYPE_COUNT]; CELL_BOX_WIDTH_COUNT]; CELL_BOX_HEIGHT_COUNT];
    let mut rng = rand::thread_rng();
    for i in 0..CELL_BOX_HEIGHT_COUNT{
        for j in 0..CELL_BOX_WIDTH_COUNT{
            for k in 0..CELL_TYPE_COUNT{
                cell_viabilities_before[i][j][k] = rng.gen();
            }
        }
    }

    let window_width = cell_size*CELL_BOX_WIDTH_COUNT as u32;
    let window_height = cell_size*CELL_BOX_HEIGHT_COUNT as u32;
    let (mut canvas, mut event_pump) 
        = sdl_modules::sdl_setup(window_width, window_height);

    // イベントループ
    'running: loop {
        if sdl_modules::is_end_event(&mut event_pump) { break 'running; }
        
        // キャンバスの初期化
        canvas.set_draw_color(Color::RGB(28, 33, 40));
        canvas.clear();

        for i in 0..CELL_BOX_HEIGHT_COUNT{
            for j in 0..CELL_BOX_WIDTH_COUNT{
                for k in 0..CELL_TYPE_COUNT{
                    // beforeを元にしてafterを作成
                    let mut count: u32 = 0;
                    for x in 0..3{
                        for y in 0..3{
                            if x == 1 && y == 1{ continue; }
                            let x_index: i32 = j as i32 + x - 1;
                            let y_index: i32 = i as i32 + y - 1;
                            if x_index < 0 || x_index >= CELL_BOX_WIDTH_COUNT as i32 || y_index < 0 || y_index >= CELL_BOX_HEIGHT_COUNT as i32{ continue; }
                            if cell_viabilities_before[y_index as usize][x_index as usize][k] == true{
                                count += 1;
                            }
                        }
                    }
                    if cell_viabilities_before[i][j][k] == true{
                        if count == 2 || count == 3{
                            cell_viabilities_after[i][j][k] = true;
                        }else{
                            cell_viabilities_after[i][j][k] = false;
                        }
                    }else{
                        if count == 3{
                            cell_viabilities_after[i][j][k] = true;
                        }else{
                            cell_viabilities_after[i][j][k] = false;
                        }
                    }
                }
            }
        }
        
        
        for i in 0..CELL_BOX_HEIGHT_COUNT{
            for j in 0..CELL_BOX_WIDTH_COUNT{
                for k in 0..CELL_TYPE_COUNT{
                    if cell_viabilities_after[i][j][k] == true{
                        let x: u32 = (j * cell_size as usize) as u32;
                        let y: u32 = (i * cell_size as usize) as u32;
                        cell[k].set_position(x, y);
                        cell[k].draw(&mut canvas);
                    }
                }
            }
        }

        // afterの内容をbeforeにコピー
        // afterのさすアドレスとbeforeのさすアドレスを交換
        std::mem::swap(&mut cell_viabilities_before, &mut cell_viabilities_after);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
