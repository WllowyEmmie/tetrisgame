extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate rodio;

use ::std::fs::File;
use ::std::io::BufReader;
use glutin_window::GlutinWindow as Window;
use graphics::types::Width;
use graphics::Context;
use graphics::*;
use graphics::{grid, math};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{EventLoop, Key, MouseButton, MouseCursorEvent};
use piston_window::types::Color;
use piston_window::{G2d, TextureSettings};
use piston_window::{Glyphs, PistonWindow};
use rand::Rng;
use rodio::source::Source;
use rodio::{Decoder, OutputStream, Sink};
use std::arch::x86_64;
use std::os::windows;
use std::process::exit;
use std::slice::Windows;
use std::time::{self, Duration, Instant};
use std::{thread, vec};

const GRID_WIDTH: usize = 10;

const GRID_HEIGHT: usize = 20;
const BLOCK_SIZE: f64 = 30.0;

#[derive(Clone, Copy)]
struct Block {
    x: i32,
    y: i32,
}
#[derive(Clone)]
struct Piece {
    blocks: Vec<Block>,
    color: [f32; 4],
}
struct Game {
    grid: Vec<Vec<Option<[f32; 4]>>>,
    current_piece: Piece,
    piece_fall_timer: Instant, // Separate timer for piece descent
    piece_settled: bool,
    new_piece_timer: Instant,
}
impl Game {
    fn new() -> Self {
        Self {
            grid: vec![vec![None; GRID_WIDTH]; GRID_HEIGHT],
            current_piece: Game::random_piece(),
            piece_fall_timer: Instant::now(), // Initialize piece fall timer
            piece_settled: false,
            new_piece_timer: Instant::now(),
        }
    }
    fn random_piece() -> Piece {
        let center_x: i32 = (GRID_WIDTH / 4) as i32;
        let shapes = vec![
            vec![
                Block {
                    x: center_x + 1,
                    y: 0,
                },
                Block {
                    x: center_x + 0,
                    y: 0,
                },
                Block {
                    x: center_x - 1,
                    y: 0,
                },
                Block {
                    x: center_x + 0,
                    y: 1,
                },
            ], // T shape
            vec![
                Block {
                    x: center_x + 0,
                    y: 0,
                },
                Block {
                    x: center_x + 1,
                    y: 0,
                },
                Block {
                    x: center_x - 1,
                    y: 0,
                },
                Block {
                    x: center_x + -1,
                    y: 1,
                },
            ], // L shape
            vec![
                Block {
                    x: center_x + 0,
                    y: 0,
                },
                Block {
                    x: center_x - 1,
                    y: 0,
                },
                Block {
                    x: center_x + 1,
                    y: 0,
                },
                Block {
                    x: center_x + 1,
                    y: 1,
                },
            ], // J shape
            vec![
                Block {
                    x: center_x + 1,
                    y: 0,
                },
                Block {
                    x: center_x + 0,
                    y: 0,
                },
                Block {
                    x: center_x + 0,
                    y: 1,
                },
                Block {
                    x: center_x + 1,
                    y: 1,
                },
            ], // O shape
            vec![
                Block {
                    x: center_x + 0,
                    y: 0,
                },
                Block {
                    x: center_x - 1,
                    y: 0,
                },
                Block {
                    x: center_x + 1,
                    y: 0,
                },
                Block {
                    x: center_x + 2,
                    y: 0,
                },
            ], // I shape
            vec![
                Block {
                    x: center_x + 0,
                    y: 0,
                },
                Block {
                    x: center_x + -1,
                    y: 0,
                },
                Block {
                    x: center_x + 0,
                    y: 1,
                },
                Block {
                    x: center_x + 1,
                    y: 1,
                },
            ], // S shape
            vec![
                Block {
                    x: center_x + 0,
                    y: 0,
                },
                Block {
                    x: center_x + 1,
                    y: 0,
                },
                Block {
                    x: center_x + 0,
                    y: 1,
                },
                Block {
                    x: center_x + -1,
                    y: 1,
                },
            ], // Z shape
        ];
        let colors = vec![
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
            [1.0, 1.0, 0.0, 1.0],
            [1.0, 0.5, 0.0, 1.0],
            [0.5, 0.0, 0.5, 1.0],
            [0.0, 1.0, 1.0, 1.0],
        ];
        let shape = shapes[rand::thread_rng().gen_range(0..shapes.len())].clone();
        let color = colors[rand::thread_rng().gen_range(0..colors.len())].clone();
        Piece {
            blocks: shape,
            color: color,
        }
    }

    fn update(&mut self) {
        // Move piece down every 500ms
        if self.piece_fall_timer.elapsed() >= Duration::from_millis(500) {
            self.piece_fall_timer = Instant::now(); // Reset the timer for piece descent

            if self.piece_settled {
                // // If the piece has settled, place it on the grid, clear rows, and spawn a new piece
                // for block in & mut self.current_piece.blocks{
                // }
                self.place_piece();
                if self.new_piece_timer.elapsed() >= Duration::from_secs(2){
                    self.current_piece = Game::random_piece();
                }
                
                println!("This is a set");
                for block in &self.current_piece.blocks {
                    println!("After place piece: X {}, Y {}", block.x, block.y);
                }

                 self.clear_full_rows();
                
                self.piece_settled = false;
            } else {
                self.moving_down();
            }
        }
    }
    fn moving_down(&mut self) {
        println!("This is a set");
        for block in &mut self.current_piece.blocks {
            block.y += 1;
            println!("Before place piece: X {}, Y {}", block.x, block.y);
        }

        // If moving down causes a collision, revert and set as settled
        if self.check_collision() {
            println!("This is a set");
            for block in &mut self.current_piece.blocks {
                println!("Before place piece after: X {}, Y {}", block.x, block.y);
                block.y -= 1; // Revert the move
            }
            self.piece_settled = true;
        }
    }

    fn clear_full_rows(&mut self) {
        // Remove rows that are completely filled
        self.grid
            .retain(|row| row.iter().any(|cell| cell.is_none()));

        // Calculate how many rows were cleared and add new empty rows at the top
        let rows_cleared = GRID_HEIGHT - self.grid.len();
        for _ in 0..rows_cleared {
            self.grid.insert(0, vec![None; GRID_WIDTH]);
        }
    }

    fn move_piece(&mut self, direction: &str) {
        match direction {
            "left" => {
                // Move left and check for collisions
                for block in &mut self.current_piece.blocks {
                    println!("The block moved left");
                    block.x -= 1;
                }
                if self.check_collision() {
                    // Revert move if collision detected
                    for block in &mut self.current_piece.blocks {
                        block.x += 1;
                    }
                }
            }
            "right" => {
                // Move right and check for collisions
                for block in &mut self.current_piece.blocks {
                    block.x += 1;
                    println!("The block moved right");
                }
                if self.check_collision() {
                    // Revert move if collision detected
                    for block in &mut self.current_piece.blocks {
                        block.x -= 1;
                    }
                }
            }
            "rotate" => {
                let center = self.current_piece.blocks[0];

                for block in &mut self.current_piece.blocks {
                    let relative_x = block.x - center.x;
                    let relative_y = block.y - center.y;

                    let rotated_x = -relative_y;
                    let rotated_y = relative_x;

                    block.x = center.x + rotated_x;
                    block.y = center.y + rotated_y;

                    println!("Rotated block to x: {}, y: {}", block.x, block.y);
                }
                if self.check_collision() {
                    for block in &mut self.current_piece.blocks {
                        let relative_x = block.x - center.x;
                        let relative_y = block.y - center.y;

                        // Revert rotation transformation (rotate back 90 degrees counter-clockwise)
                        let reverted_x = relative_y;
                        let reverted_y = -relative_x;

                        block.x = center.x + reverted_x;
                        block.y = center.y + reverted_y;
                    }
                }
            }
            _ => {}
        }
    }

    fn check_collision(&self) -> bool {
        for block in &self.current_piece.blocks {
            // Check if the block is out of bounds or overlapping with an existing cell
            if block.x < 0 || block.x as usize >= GRID_WIDTH || block.y as usize >= GRID_HEIGHT {
                return true;
            }
            if block.y >= 0 && self.grid[block.y as usize][block.x as usize].is_some() {
                return true;
            }
        }
        false
    }

    fn place_piece(&mut self) {
        let center = self.current_piece.blocks[0];
      

        for block in &self.current_piece.blocks {
        
            // Assign color to the actual position of the block
            self.grid[block.y as usize][block.x as usize] = Some(self.current_piece.color);
            
        }
    }
   
}
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tetris", [300, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Left => game.move_piece("left"),
                Key::Right => game.move_piece("right"),
                Key::Up => game.move_piece("rotate"),
                _ => {}
            };
        }
        game.update();
        if let Some(args) = event.render_args() {
            window.draw_2d(&event, |c, g, d| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                for (y, row) in game.grid.iter().enumerate() {
                    for (x, cell) in row.iter().enumerate() {
                        if let Some(color) = cell {
                            let transform = c.transform.trans(x as f64* BLOCK_SIZE, y as f64 * BLOCK_SIZE);
                            rectangle(*color, [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE], transform, g);
                        }
                    }
                }
                for block in &game.current_piece.blocks {
                    let color = game.current_piece.color;
                    let transform = c
                        .transform
                        .trans(block.x as f64 * BLOCK_SIZE, block.y as f64 * BLOCK_SIZE);
                    rectangle(color, [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE], transform, g);
                }
            });
        }
    }
}
