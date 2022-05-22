use std::time::SystemTime;

use nannou::noise::*;
use nannou::prelude::*;

const N_THINGS: usize = 2000;
fn main() {
    nannou::app(model).update(update).run()
}
struct Thing {
    position: Vec<Vec2>,
}

impl Thing {
    fn new(p: Vec2) -> Self {
        let mut position = vec![p];

        Thing { position }
    }
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .size(1920, 1080)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let mut things = Vec::new();
    for i in 0..N_THINGS {
        things.push(Thing::new(vec2(get_pos(), get_pos())));
    }
    let noise = Perlin::new();
    Model { things, noise }
}

fn get_pos() -> f32 {
    (random::<f32>() - 0.5) * 720.0
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 60.0;

    let sn = 0.01 + time.cos() as f64 * 0.007;
    for thing in model.things.iter_mut() {
        thing.position.clear();
        thing.position.push(vec2(get_pos(), get_pos()));
        for _ in 0..60 {
            let last = thing.position[0];
            let new = last
                + vec2(
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 0.0])
                        as f32,
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 1.0])
                        as f32,
                );
            thing.position.insert(0, new);
        }
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .w_h(1920.0, 1080.0)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for thing in model.things.iter() {
        draw.polygon()
            .points(thing.position.iter().cloned())
            .color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap()
}
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
