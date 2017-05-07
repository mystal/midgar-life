#![feature(conservative_impl_trait)]
#![feature(test)]

extern crate cgmath;
extern crate midgar;

use midgar::{KeyCode, Midgar, Mouse, Surface};
use midgar::graphics::shape::ShapeRenderer;

use std::time::{Duration, Instant};

mod board;


const SCREEN_SIZE: (u32, u32) = (600, 600);
const BOARD_SIZE: (u32, u32) = (60, 60);
const CELL_SIZE: (u32, u32) = (SCREEN_SIZE.0 / BOARD_SIZE.0, SCREEN_SIZE.1 / BOARD_SIZE.1);
const STEP_TIME_MS: u64 = 200;

const WHITE: [f32; 3] = [1.0, 1.0, 1.0];


pub struct LifeApp {
    step_time: Duration,

    shape_renderer: ShapeRenderer,
    projection: cgmath::Matrix4<f32>,
    board: board::LifeBoard,

    simulate: bool,
    last_step_time: Instant,
    clearing: bool,
    selected_cell: Option<board::Cell>,
}

impl LifeApp {
    fn draw_board<S: Surface>(&self, midgar: &Midgar, target: &mut S) {
        for cell in self.board.iter_live_cells() {
            // TODO: Check that cell is in visible boundary
            // Don't process negative cells to avoid crashing due to overflow
            if cell.x < 0 || cell.y < 0 {
                continue;
            }
            let x = cell.x as u32 * CELL_SIZE.0;
            let y = cell.y as u32 * CELL_SIZE.1;
            self.shape_renderer.draw_filled_rect(x as f32, y as f32, CELL_SIZE.0 as f32, CELL_SIZE.1 as f32, WHITE,
                                                 target);
        }
    }
}

impl midgar::App for LifeApp {
    fn create(midgar: &Midgar) -> Self {
        let mut b = board::LifeBoard::new();
        b.set(1, 0, true);
        b.set(2, 1, true);
        b.set(0, 2, true);
        b.set(1, 2, true);
        b.set(2, 2, true);

        let projection = cgmath::ortho(0.0, SCREEN_SIZE.0 as f32, 0.0, SCREEN_SIZE.1 as f32, -1.0, 1.0);

        LifeApp {
            step_time: Duration::from_millis(STEP_TIME_MS),

            shape_renderer: ShapeRenderer::new(midgar.graphics().display(), projection),
            projection: projection,
            board: b,

            simulate: false,
            last_step_time: Instant::now(),
            clearing: false,
            selected_cell: None,
        }
    }

    fn step(&mut self, midgar: &mut Midgar) {
        if midgar.input().was_key_pressed(&KeyCode::Escape) {
            midgar.set_should_exit();
            return;
        }

        self.selected_cell = {
            let (x, y) = midgar.input().mouse_pos();
            let y = SCREEN_SIZE.1 as i32 - y;
            if x >= 0 && y >= 0 {
                let x = x as u32 / CELL_SIZE.0;
                let y = y as u32 / CELL_SIZE.1;
                Some(board::Cell::new(x as i64, y as i64))
            } else {
                None
            }
        };

        if midgar.input().was_button_pressed(&Mouse::Left) {
            if let Some(cell) = self.selected_cell {
                self.clearing = self.board.get(cell.x as i64, cell.y as i64);
            }
        }

        if midgar.input().is_button_held(&Mouse::Left) {
            if let Some(cell) = self.selected_cell {
                self.board.set(cell.x as i64, cell.y as i64, !self.clearing);
            }
        }

        let mut step_board = false;
        if midgar.input().was_key_pressed(&KeyCode::Space) {
            self.simulate = !self.simulate;
        }

        if midgar.input().was_key_pressed(&KeyCode::S) {
            step_board = true;
        }

        if midgar.input().was_key_pressed(&KeyCode::C) {
            self.board.clear();
        }

        let last_step_dt = Instant::now() - self.last_step_time;
        if self.simulate && (last_step_dt >= self.step_time) {
            step_board = true;
        }

        if step_board {
            self.board.step();
            self.last_step_time = Instant::now();
        }

        let mut target = midgar.graphics().display().draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        self.draw_board(midgar, &mut target);
        target.finish().unwrap();
    }
}


fn main() {
    let config = midgar::MidgarAppConfig::new()
        .with_title("midgar-life")
        .with_screen_size(SCREEN_SIZE);
    let app: midgar::MidgarApp<LifeApp> = midgar::MidgarApp::new(config);
    app.run();
}
