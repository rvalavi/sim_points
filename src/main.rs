use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Point {
    pos: Vec2,
    vel: Vec2,
}

fn rand_vel() -> Vec2 {
    // random direction + speed
    let angle = rand::gen_range(0.0, std::f32::consts::TAU);
    let speed = rand::gen_range(30.0, 120.0); // px/s
    vec2(angle.cos(), angle.sin()) * speed
}

fn bounce(pos: &mut Vec2, vel: &mut Vec2, w: f32, h: f32) {
    if pos.x < 0.0 {
        pos.x = 0.0;
        vel.x = vel.x.abs();
    } else if pos.x > w {
        pos.x = w;
        vel.x = -vel.x.abs();
    }

    if pos.y < 0.0 {
        pos.y = 0.0;
        vel.y = vel.y.abs();
    } else if pos.y > h {
        pos.y = h;
        vel.y = -vel.y.abs();
    }
}

#[macroquad::main("Points")]
async fn main() {
    let n = 200usize;

    let mut points: Vec<Point> = (0..n)
        .map(|_| Point {
            pos: vec2(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height()),
            ),
            vel: rand_vel(),
        })
        .collect();

    loop {
        let dt = get_frame_time();
        let w = screen_width();
        let h = screen_height();

        // Update
        for p in &mut points {
            p.pos += p.vel * dt;
            bounce(&mut p.pos, &mut p.vel, w, h);
        }

        // Draw
        clear_background(BLACK);
        for p in &points {
            draw_circle(p.pos.x, p.pos.y, 3.0, WHITE);
        }

        // Optional: reset with R
        if is_key_pressed(KeyCode::R) {
            for p in &mut points {
                p.pos = vec2(rand::gen_range(0.0, w), rand::gen_range(0.0, h));
                p.vel = rand_vel();
            }
        }

        next_frame().await
    }
}
