use nannou::color::{Gradient, IntoLinSrgba};
use nannou::prelude::*;
use std::ops::Div;
use std::time::SystemTime;

// background DARK NAVY BLUE 2a2a36

const INNER_BORDER: Srgb<u8> = WHITE;
// Line B gradient-like or textured taupe or beige shade cec3bd
const LINE_COLOR_B: Srgb<u8> = Srgb {
    red: 206,
    green: 195,
    blue: 189,
    standard: core::marker::PhantomData,
};

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run()
}

struct Model {
    random_seed: u64,
    etole: Etole,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(600, 900)
        .title(app.exe_name().unwrap())
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let seed = "ysl"
        .as_bytes()
        .len()
        .to_u64()
        .expect("Failed to convert seed to u64");

    Model {
        random_seed: seed,
        etole: Etole::new_fake(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::D => {
            model.etole.debug = !model.etole.debug;
        }
        Key::N => {
            model.etole = Etole::new_fake();
        }
        Key::M => {
            model.etole = Etole::new_real_paysage();
        }
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
        Key::P => {
            model.etole.mode = Mode::Dot;
        }
        Key::O => {
            model.etole.mode = Mode::Wave;
        }
        Key::I => {
            model.etole.mode = Mode::Line;
        }
        _other_key => {
            println!("{:?}", _other_key)
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = Draw::new();

    draw.background();
    model.etole.draw(&draw);
    // draw W letter

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn w_letter(inner_w: f32, inner_h: f32) -> Vec<Point2> {
    vec![
        pt2(259.00, 340.00),
        pt2(261.12, 322.96),
        pt2(266.74, 298.00),
        pt2(277.26, 267.00),
        pt2(293.56, 211.00),
        pt2(312.02, 150.00),
        pt2(319.98, 124.00),
        pt2(325.33, 106.00),
        pt2(330.49, 93.30),
        pt2(327.42, 89.90),
        pt2(313.74, 82.21),
        pt2(311.00, 81.41),
        pt2(305.97, 79.53),
        pt2(304.09, 81.41),
        pt2(298.68, 97.11),
        pt2(297.86, 100.00),
        pt2(284.98, 143.00),
        pt2(271.28, 189.00),
        pt2(249.97, 259.00),
        pt2(238.49, 294.00),
        pt2(236.72, 299.28),
        pt2(234.00, 304.63),
        pt2(233.00, 310.00),
        pt2(218.87, 306.78),
        pt2(207.37, 296.66),
        pt2(199.00, 294.00),
        pt2(199.00, 292.00),
        pt2(218.00, 279.67),
        pt2(225.30, 272.91),
        pt2(223.69, 265.00),
        pt2(218.00, 249.00),
        pt2(212.69, 251.53),
        pt2(211.75, 253.88),
        pt2(207.91, 258.00),
        pt2(193.00, 273.00),
        pt2(190.50, 275.49),
        pt2(185.80, 280.96),
        pt2(182.00, 280.65),
        pt2(179.19, 280.42),
        pt2(174.23, 276.19),
        pt2(172.00, 274.39),
        pt2(166.83, 270.21),
        pt2(157.32, 261.85),
        pt2(152.00, 259.00),
        pt2(148.27, 277.00),
        pt2(148.27, 284.37),
        pt2(154.00, 288.25),
        pt2(168.00, 296.00),
        pt2(145.00, 310.00),
        pt2(132.31, 286.00),
        pt2(123.31, 267.00),
        pt2(86.31, 187.00),
        pt2(66.25, 143.00),
        pt2(48.26, 104.00),
        pt2(46.42, 100.20),
        pt2(44.36, 91.89),
        pt2(38.98, 92.84),
        pt2(36.88, 93.21),
        pt2(32.88, 95.93),
        pt2(31.00, 97.17),
        pt2(27.79, 99.29),
        pt2(20.75, 103.33),
        pt2(19.70, 107.04),
        pt2(18.79, 110.25),
        pt2(21.85, 115.09),
        pt2(23.22, 118.00),
        pt2(35.14, 144.00),
        pt2(64.69, 208.00),
        pt2(93.32, 272.00),
        pt2(102.75, 290.00),
        pt2(108.73, 301.96),
        pt2(114.01, 314.21),
        pt2(117.98, 327.00),
        pt2(118.82, 329.73),
        pt2(120.93, 340.73),
        pt2(122.73, 341.95),
        pt2(126.04, 344.19),
        pt2(136.14, 335.72),
        pt2(139.00, 333.72),
        pt2(171.00, 312.00),
        pt2(173.96, 310.02),
        pt2(180.83, 304.50),
        pt2(184.00, 304.33),
        pt2(187.31, 304.16),
        pt2(201.18, 311.85),
        pt2(205.00, 313.75),
        pt2(238.00, 330.25),
        pt2(243.90, 333.20),
        pt2(253.02, 338.29),
        pt2(259.00, 340.00),
    ]
}

fn y_letter(inner_w: f32, inner_h: f32) -> Vec<Point2> {
    let y_points = vec![
        pt2(-inner_w.div(2.0), inner_h.div(2.0)),
        pt2(-inner_w.div(4.0), inner_h.div(4.0)),
        //pt2(0.0, inner_h.div(2.0)),
        pt2(-inner_w.div(4.0), inner_h.div(10.0)),
        //pt2(inner_w.div(4.0), inner_h.div(4.0)),
        pt2(inner_w.div(2.0), inner_h.div(2.0)),
    ];
    y_points
}

pub struct Dimension {
    pub width: f32,
    pub height: f32,
    pub stroke_weight: f32,
    margin: Option<i32>,
}
enum Mode {
    Line,
    Dot,
    Wave,
}
pub struct Etole {
    outer_border: Dimension,
    inner_border: Dimension,
    line: Dimension,
    background_color: Srgb<u8>,
    outer_border_color: Srgb<u8>,
    inner_border_color: Srgb<u8>,
    line_color_a: Srgb<u8>,
    mode: Mode,
    debug: bool,
}

impl Etole {
    fn new_fake() -> Self {
        Etole {
            mode: Mode::Line,
            outer_border: Dimension {
                width: 300.0,
                height: 450.0,
                // ) pixel
                stroke_weight: 3.0,
                margin: None,
            },
            inner_border: Dimension {
                width: 600f32 / 2.5,
                height: 900f32 / 2.5,
                // ) pixel
                stroke_weight: 3.0,
                margin: Some(3),
            },
            line: Dimension {
                width: 600f32 / 2.5 / 2.0,
                height: 900f32 / 2.5 / 2.0,
                // ) pixel
                stroke_weight: 3.0,
                margin: Some(3),
            },

            // background DARK NAVY BLUE 2a2a36
            background_color: Srgb {
                red: 42,
                green: 42,
                blue: 54,
                standard: core::marker::PhantomData,
            },
            // Line B gradient-like or textured taupe or beige shade cec3bd
            outer_border_color: Srgb {
                red: 214,
                green: 199,
                blue: 186,
                standard: core::marker::PhantomData,
            },
            inner_border_color: WHITE,
            // Line A gradient-like or textured taupe or beige shade cec3bd
            line_color_a: Srgb {
                red: 186,
                green: 166,
                blue: 142,
                standard: core::marker::PhantomData,
            },
            debug: false,
        }
    }
    fn new_real() -> Self {
        //     // line 682
        Etole {
            mode: Mode::Line,
            outer_border: Dimension {
                width: 823.0,
                height: 1020.0,
                // ) pixel
                stroke_weight: 9.0,
                margin: None,
            },
            inner_border: Dimension {
                width: 694.0,
                height: 888.0,
                // ) pixel
                stroke_weight: 4.0,
                margin: None,
            },
            line: Dimension {
                width: 694.0 / 2.0,
                height: 888.0 / 2.0,
                // ) pixel
                stroke_weight: 4.0,
                margin: Some(3),
            },

            // background DARK NAVY BLUE 2a2a36
            background_color: Srgb {
                red: 42,
                green: 42,
                blue: 54,
                standard: core::marker::PhantomData,
            },
            // Line B gradient-like or textured taupe or beige shade cec3bd
            outer_border_color: Srgb {
                red: 214,
                green: 199,
                blue: 186,
                standard: core::marker::PhantomData,
            },
            inner_border_color: WHITE,
            // Line A gradient-like or textured taupe or beige shade cec3bd
            line_color_a: Srgb {
                red: 186,
                green: 166,
                blue: 142,
                standard: core::marker::PhantomData,
            },
            debug: false,
        }
    }
    fn new_real_paysage() -> Self {
        //     // line 682

        Etole {
            mode: Mode::Line,
            outer_border: Dimension {
                width: 1020.0,
                height: 823.0,
                // ) pixel
                stroke_weight: 9.0,
                margin: None,
            },
            inner_border: Dimension {
                width: 888.0,
                height: 694.0,
                // ) pixel
                stroke_weight: 4.0,
                margin: None,
            },
            line: Dimension {
                width: 888.0 / 2.0,
                height: 694.0 / 2.0,
                // ) pixel
                stroke_weight: 4.0,
                margin: Some(3),
            },

            // background DARK NAVY BLUE 2a2a36
            background_color: Srgb {
                red: 42,
                green: 42,
                blue: 54,
                standard: core::marker::PhantomData,
            },
            // Line B gradient-like or textured taupe or beige shade cec3bd
            outer_border_color: Srgb {
                red: 214,
                green: 199,
                blue: 186,
                standard: core::marker::PhantomData,
            },
            inner_border_color: WHITE,
            // Line A gradient-like or textured taupe or beige shade cec3bd
            line_color_a: Srgb {
                red: 186,
                green: 166,
                blue: 142,
                standard: core::marker::PhantomData,
            },
            debug: false,
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.background().color(WHITE);
        draw.rect()
            .w_h(self.outer_border.width, self.outer_border.height)
            .color(self.background_color);

        draw.rect()
            .w_h(self.outer_border.width, self.outer_border.height)
            .stroke_weight(self.outer_border.stroke_weight)
            .stroke(self.outer_border_color)
            .color(self.outer_border_color)
            .no_fill();

        draw.rect()
            .w_h(self.inner_border.width, self.inner_border.height)
            .stroke_weight(self.inner_border.stroke_weight)
            .stroke(self.inner_border_color)
            .no_fill();

        self.line(draw);

        let center = pt2(-171.0, -521.0 / 2.0);

        draw.polygon()
            .points(w_letter(self.line.width, self.line.height))
            .xy(-center)
            .rotate(PI)
            .color(self.background_color);
        if self.debug {
            let r = Rect::from_w_h(342.0, 521.0);

            draw.rect()
                .w_h(r.w(), r.h())
                .stroke_weight(self.line.stroke_weight)
                .stroke(self.line_color_a)
                .no_fill();
            draw.ellipse().radius(10.0).xy(center);
        }
    }

    fn line(&self, draw: &Draw) {
        let gradient_b = Gradient::new(vec![
            self.line_color_a.into_lin_srgba(),
            WHITE.into_lin_srgba(),
            //self.background_color.into_lin_srgba(),
        ]);
        // Secondary gradient
        let gradient = Gradient::new(vec![
            DEEPPINK.into_lin_srgba(),
            GREEN.into_lin_srgba(),
            self.line_color_a.into_lin_srgba(),
            YELLOW.into_lin_srgba(),
            //self.background_color.into_lin_srgba(),
        ]);

        let margin = self.line.margin.unwrap();
        let min_height = -self.line.height as i32 + margin;
        let max_height = self.line.height as i32;
        let min_width = -self.line.width as i32 + margin;
        let max_width = self.line.width as i32;
        match self.mode {
            Mode::Line => {
                for i in min_height..max_height {
                    // create different shade  of gradient
                    // From a to white

                    if i % 3 == 0 {
                        let y = i as f32;
                        let c = gradient.get(random_range(0.0f64, 1.0f64));

                        draw.line()
                            .start(pt2(-self.line.width + margin as f32, y))
                            .end(pt2(self.line.width - margin as f32, y))
                            .stroke_weight(0.5)
                            .color(self.line_color_a);
                    }
                }
            }
            Mode::Wave => {
                for i in min_height..max_height {
                    if i % 5 == 0 {
                        let i1 = random_range(0, 100);
                        let y = i as f32;
                        let end = max_width - min_width;
                        let alpha1 = self.line_color_a;
                        let points = (0..=end).map(|i| {
                            let x = (i as f32) + min_width as f32; // scale the integer to a float

                            if i1 > 20 {
                                let point = pt2(x, y);
                                (point, alpha1)
                            } else {
                                let x1 = x.sin();
                                let point = pt2(x, x1 + y);
                                (point, alpha1)
                            }
                        });
                        draw.polyline().points_colored(points);
                    }
                }
            }

            Mode::Dot => {
                for i in min_height..max_height {
                    if i % 5 == 0 {
                        for k in min_width..max_width {
                            if k % 6 == 0 {
                                let x_shift = if i % 2 == 0 { 0 } else { 3 } + k;
                                let alpha = gradient_b
                                    .get(random_range(min_height as f64, max_height as f64));
                                let h = random_range(1.0, 3.0);

                                draw.rect()
                                    .w_h(h, h)
                                    .x_y(x_shift as f32, i as f32)
                                    .rotate(random_range(0.0, PI))
                                    .color(alpha);
                            }
                        }
                    }
                }
            }
        }
    }
}
