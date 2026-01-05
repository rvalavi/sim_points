use macroquad::prelude::*;
use std::env;

#[derive(Clone, Copy)]
struct Point {
    pos: Vec2,
    vel: Vec2,
}

fn rand_vel() -> Vec2 {
    let angle = rand::gen_range(0.0, std::f32::consts::TAU);
    let speed = rand::gen_range(40.0, 120.0);
    vec2(angle.cos(), angle.sin()) * speed
}

fn reset_points(points: &mut Vec<Point>, n: usize, r: f32, w: f32, h: f32) {
    points.clear();
    points.reserve(n);
    for _ in 0..n {
        points.push(Point {
            pos: vec2(rand::gen_range(r, w - r), rand::gen_range(r, h - r)),
            vel: rand_vel(),
        });
    }
}

fn bounce_walls(p: &mut Point, w: f32, h: f32, r: f32) {
    if p.pos.x < r {
        p.pos.x = r;
        p.vel.x = p.vel.x.abs();
    } else if p.pos.x > w - r {
        p.pos.x = w - r;
        p.vel.x = -p.vel.x.abs();
    }

    if p.pos.y < r {
        p.pos.y = r;
        p.vel.y = p.vel.y.abs();
    } else if p.pos.y > h - r {
        p.pos.y = h - r;
        p.vel.y = -p.vel.y.abs();
    }
}

// equal-mass, perfectly elastic collision between two discs
fn collide(a: &mut Point, b: &mut Point, r: f32) {
    let delta = b.pos - a.pos;
    let dist2 = delta.length_squared();
    let min_dist = 2.0 * r;

    if dist2 >= min_dist * min_dist {
        return;
    }

    let dist = dist2.sqrt().max(1e-6);
    let n = delta / dist;

    // separate overlap
    let penetration = min_dist - dist;
    let correction = 0.5 * penetration * n;
    a.pos -= correction;
    b.pos += correction;

    // impulse (only if moving toward each other)
    let rel_vel = b.vel - a.vel;
    let rel_normal_speed = rel_vel.dot(n);
    if rel_normal_speed > 0.0 {
        return;
    }

    // equal masses, restitution=1 => j = -rel_normal_speed
    let j = -rel_normal_speed;
    let impulse = j * n;

    a.vel -= impulse;
    b.vel += impulse;
}

#[macroquad::main("Points")]
async fn main() {
    // let n = 200usize;
    let n: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(200);
    let r = 3.0;
    let mut points: Vec<Point> = Vec::with_capacity(n);

    // initial reset (use current screen size)
    reset_points(&mut points, n, r, screen_width(), screen_height());

    loop {
        let dt = get_frame_time();
        let w = screen_width();
        let h = screen_height();

        if is_key_pressed(KeyCode::R) {
            reset_points(&mut points, n, r, w, h);
        }

        // move + wall bounce
        for p in &mut points {
            p.pos += p.vel * dt;
            bounce_walls(p, w, h, r);
        }

        // pairwise collisions (O(n^2))
        let len = points.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (left, right) = points.split_at_mut(j);
                let a = &mut left[i];
                let b = &mut right[0];
                collide(a, b, r);
            }
        }

        clear_background(BLACK);
        for p in &points {
            draw_circle(p.pos.x, p.pos.y, r, WHITE);
        }

        next_frame().await
    }
}
