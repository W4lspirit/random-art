use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8};
use std::time::SystemTime;

use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const WIDTH_F: u32 = 1920;
const HEIGHT_F: u32 = 1080;
const SIZE: u32 = 30;
const COLS: u32 = (WIDTH_F - 2 * MARGIN) / SIZE;
const ROWS: u32 = (HEIGHT_F - 2 * MARGIN) / SIZE;
const MARGIN: u32 = 35;

const LINE_WIDTH: f32 = 0.06;
fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run()
}
struct Model {
    random_seed: u64,
}
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH_F, HEIGHT_F)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let seed = "the seed is now "
        .as_bytes()
        .len()
        .to_u64()
        .expect("Failed to convert seed to u64");

    Model { random_seed: seed }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1000000);
        }
        Key::S => {
            let instant = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + &instant + ".png");
        }
        _other_key => {
            println!("{:?}", _other_key)
        }
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = Draw::new();

    draw.background().color(WHITE);

    let draw = draw.scale(SIZE as f32).scale_y(-1.0);
    let gdraw = draw.x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    schotter(model, gdraw);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

const GOLD: f32 = 1.61803398875;

fn schotter(model: &Model, gdraw: Draw) {
    let mut rng = StdRng::seed_from_u64(model.random_seed);
    for y in 0..ROWS {
        for x in 0..COLS {
            let cdraw = gdraw.x_y(x as f32, y as f32);
            let factor = y as f32 / ROWS as f32;
            let x_offset = factor * rng.gen_range(-0.3..0.3);
            let y_offset = factor * rng.gen_range(-0.3..0.3);
            let rotation = FRAC_PI_4 * rng.gen_range(-PI / 5.0..PI / 5.0);
            let yaw = GOLD * factor * rng.gen_range(-PI / 2.0..PI / 2.0);
            cdraw
                .quad()
                .no_fill()
                .stroke_color(BLACK)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(x_offset, y_offset)
                .rotate(rotation)
                .yaw(FRAC_PI_3);

            let i = rng.gen_range(5..10);

            for r_y in 0..i {
                let sub_range = rng.gen_range(0.0..0.6);
                let x1 = FRAC_PI_6 * r_y as f32;
                cdraw
                    .quad()
                    .no_fill()
                    .stroke_color(srgba(0.0, 0.0, 0.0, rng.gen_range(0.5..0.9)))
                    .stroke_weight(LINE_WIDTH)
                    .w_h(sub_range, sub_range)
                    .x_y(x_offset, y_offset)
                    .yaw(x1)
                    .rotate(rotation);
            }
        }
    }
}
