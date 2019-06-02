extern crate graphics;

extern crate piston_window;

extern crate rand;

use graphics::math::{add, mul_scalar, Vec2d};

use piston_window::{clear, rectangle, PistonWindow, WindowSettings};

use rand::distributions::{IndependentSample, Range};

type RGBA = [f32; 4];

const WHITE: RGBA = [1.0; 4];
const GRAY: RGBA = [0.7, 0.7, 0.7, 0.3];
const N_PARTICLES: isize = 500;

struct World {
    current_turn: usize,
    shapes: Vec<Box<Shape>>,
    height: u32,
    width: u32,
}

struct Shape {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: RGBA,
}

main

impl Shape {
    fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();

        let legal_range = Range::new(-5_f64, 5_f64);

        let x_speed = legal_range.ind_sample(&mut rng);
        let y_speed = legal_range.ind_sample(&mut rng);
        let x_accel = 0.1 * legal_range.ind_sample(&mut rng);
        let y_accel = 0.1 * legal_range.ind_sample(&mut rng);

        Shape {
            height: 10.0,
            width: 10.0,
            position: [x, y],
            velocity: [x_speed, y_speed],
            acceleration: [x_accel, y_accel],
            color: GRAY,
        }
    }

    fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration);

        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, 0.7);

        self.color[3] *= 0.97;
    }
}

impl World {
    fn new(width: u32, height: u32) -> World {
        World {
            current_turn: 0,
            shapes: Vec::<Box<Shape>>::new(),
            height: height,
            width: width,
        }
    }

    fn add_shapes(&mut self, n: usize) {
        let x = (self.width / 2) as f64;
        let y = (self.height / 2) as f64;

        for _ in 0..n {
            self.shapes.push(Box::new(Shape::new(x, y)));
        }
    }

    fn remove_shapes(&mut self, n: usize) {
        let n_shapes = self.shapes.len();

        let to_remove = if n > n_shapes { n_shapes } else { n };

        for _ in 0..to_remove {
            self.shapes.remove(0);
        }

        self.shapes.shrink_to_fit();
    }

    fn calc_population_change(&self) -> isize {
        const N: f64 = N_PARTICLES as f64;

        const MAX: f64 = N * 0.5;
        const MIN: f64 = -N * 0.5;
        let x: f64 = self.current_turn as f64;

        let n = 0.4 * N * (0.1 * x).sin() + 0.1 * N * x.sin();
        n.max(MIN).min(MAX).round() as isize
    }

    fn update(&mut self) {
        let n = self.calc_population_change();

        if n > 0 {
            self.add_shapes(n as usize);
        } else {
            self.remove_shapes(n.abs() as usize);
        }

        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (640, 480);

    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window.");

    let mut world = World::new(width, height);

    world.add_shapes(N_PARTICLES);

    while let Some(event) = window.next() {
        for shape in &mut world.shapes {
            shape.update();
        }

        world.update();

        window.draw_2d(&event, |ctx, renderer| {
            clear(WHITE, renderer);

            for s in &mut world.shapes {
                let rect = [s.position[0], s.position[1], s.width, s.height];

                let transformation_matrix = ctx.transform;

                rectangle(s.color, rect, transformation_matrix, renderer);
            }
        });
    }
}
