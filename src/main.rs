use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::{thread, time::Duration};

const THICKNESS: f32 = 1.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Walker with Line Trail".to_owned(),
        window_width: 3840,
        window_height: 2160,
        fullscreen: false,
        ..Default::default()
    }
}

struct Walker {
    path: Vec<Vec2>, // Массив всех точек пути
}

impl Walker {
    fn new() -> Self {
        let start = vec2(screen_width() / 2.0, screen_height() / 2.0);
        Self { path: vec![start] }
    }

    fn step(&mut self) {
        let last = *self.path.last().unwrap();

        let dx = gen_range(-1.0, 1.0);
        let dy = gen_range(-1.0, 1.0);

        let mut new_pos = last + vec2(dx, dy);

        // Ограничим экраном
        new_pos.x = new_pos.x.clamp(0.0, screen_width());
        new_pos.y = new_pos.y.clamp(0.0, screen_height());

        self.path.push(new_pos);
    }

    fn draw(&self) {
        thread::sleep(Duration::from_millis(1));

        for i in 1..self.path.len() {
            let a = self.path[i - 1];
            let b = self.path[i];
            draw_line(a.x, a.y, b.x, b.y, THICKNESS, WHITE);
        }
    }
}

#[macroquad::main(window_conf, "Persistent Trail")]
async fn main() {
    let mut walker = Walker::new();

    loop {
        walker.step();
        walker.draw();

        next_frame().await;
    }
}
