use std::time::SystemTime;

use nannou::color;
use nannou::draw::Drawing;
use nannou::prelude::*;
use nannou::rand::prelude::SliceRandom;
use nannou::rand::rngs::StdRng;
use nannou::rand::{thread_rng, Rng, SeedableRng};

const LINE_WIDTH: f32 = 0.06;

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 900;
const STRUCTURE_WIDTH: f32 = 60.0;
const STRUCTURE_HEIGHT: f32 = 80.0;
const COLS: i32 = 7;
const ROWS: i32 = 9;
const MAX_SHADES: usize = 63;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run()
}

struct Model {
    zones: Vec<Zone>,
    shades_number: usize,
}

struct Zone {
    position: Vec2,
    color: Rgb,
}

impl Zone {
    pub fn new(i_x: f32, i_y: f32) -> Self {
        let w = STRUCTURE_WIDTH;
        let h = STRUCTURE_HEIGHT;
        let max_x = WINDOW_WIDTH as f32;
        let max_y = WINDOW_HEIGHT as f32;

        let x = i_x * w;
        let y = i_y * h;
        let x = x - (max_x * 0.3);
        let y = y - (max_y * 0.35);

        let position = vec2(x, y);

        Zone {
            position,
            color: Zone::rando_color(),
        }
    }
    pub fn rando_color() -> Rgb {
        let colors = vec![
            rgb(0.38, 0.68, 0.67), // green
            rgb(0.1, 0.43, 0.65),  // blue
            rgb(0.65, 0.54, 0.68), // purple
            rgb(0.92, 0.54, 0.38), // orange
            rgb(0.24, 0.24, 0.24), // black
        ];
        let color = colors.choose(&mut thread_rng()).unwrap();
        *color
    }
    pub(crate) fn set_points(&mut self) {
        self.position = self.position + (random_f32() * 0.5);
    }
    pub fn set_color(&mut self) {
        self.color = Zone::rando_color()
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(app.exe_name().unwrap())
        .view(view)
        .mouse_wheel(|_app, model: &mut Model, delta, phase| match delta {
            MouseScrollDelta::LineDelta(_, y) if y > 0.0 => {
                if model.shades_number < MAX_SHADES as usize {
                    model.shades_number += 1;
                }
            }
            MouseScrollDelta::LineDelta(_, y) if y < 0.0 => {
                if model.shades_number > 1 {
                    model.shades_number -= 1;
                }
            }
            _ => {}
        })
        .mouse_released(|_app, model: &mut Model, button| match button {
            MouseButton::Left => {
                for zone in model.zones.iter_mut() {
                    zone.set_points();
                }
            }
            MouseButton::Right => {
                for zone in model.zones.iter_mut() {
                    zone.set_color();
                }
            }
            _ => {}
        })
        .key_pressed(key_pressed)
        .build()
        .unwrap();


    let mut zones = Vec::new();
    for col in 0..COLS {
        for row in 0..ROWS {
            zones.push(Zone::new(col as f32, row as f32))
        }
    }
    Model {
        zones,
        shades_number: 1,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let instant = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            app.main_window().capture_frame(
                app.exe_name().unwrap() + "/" + &app.exe_name().unwrap() + &instant + ".png",
            );
        }
        _other_key => {
            println!("{:?}", _other_key)
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = Draw::new();

    draw.background().color(PLUM);
    let initial_size = vec2(100.0, 100.0);
    for zone in model.zones.iter().take(model.shades_number) {
        for i in 0..model.shades_number {
            let pos = zone.position;
            let size = initial_size / (1.0 + 0.9 * i as f32);
            let yaw =i as f32;
            draw.rect()
                .color(srgba(
                    random_f32(),
                    random_f32(),
                    random_f32(),
                    random_f32(),
                ))
                .xy(pos)
                .wh(size)
                .rotate(TAU / yaw);
            draw.rect()
                .color(WHITE)
                .xy(pos)
                .wh(size * 0.9)
                .rotate(-TAU / yaw);
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
