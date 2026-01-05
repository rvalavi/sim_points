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

    // impulse if approaching
    let rel_vel = b.vel - a.vel;
    let rel_normal_speed = rel_vel.dot(n);
    if rel_normal_speed > 0.0 {
        return;
    }

    let j = -rel_normal_speed; // equal mass, e=1
    let impulse = j * n;
    a.vel -= impulse;
    b.vel += impulse;
}

fn nearest_point(points: &[Point], mouse: Vec2, max_dist: f32) -> Option<usize> {
    let mut best_i: Option<usize> = None;
    let mut best_d2 = max_dist * max_dist;

    for (i, p) in points.iter().enumerate() {
        let d2 = (p.pos - mouse).length_squared();
        if d2 < best_d2 {
            best_d2 = d2;
            best_i = Some(i);
        }
    }
    best_i
}

#[macroquad::main("Points")]
async fn main() {
    let n: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(200);
    let r = 4.0;
    let pick_radius = 18.0;

    let mut points: Vec<Point> = Vec::with_capacity(n);
    reset_points(&mut points, n, r, screen_width(), screen_height());

    let mut grabbed: Option<usize> = None;
    let mut last_mouse = vec2(mouse_position().0, mouse_position().1);

    loop {
        let dt = get_frame_time();
        let w = screen_width();
        let h = screen_height();

        let mouse = vec2(mouse_position().0, mouse_position().1);
        let mouse_vel = (mouse - last_mouse) / dt.max(1e-6);
        last_mouse = mouse;

        if is_key_pressed(KeyCode::R) {
            reset_points(&mut points, n, r, w, h);
            grabbed = None;
        }

        // start grab
        if is_mouse_button_pressed(MouseButton::Left) {
            grabbed = nearest_point(&points, mouse, pick_radius);
        }

        // release grab
        if is_mouse_button_released(MouseButton::Left) {
            grabbed = None;
        }

        // physics update
        for p in &mut points {
            p.pos += p.vel * dt;
            bounce_walls(p, w, h, r);
        }

        // if grabbing, override that particle AFTER motion so it sticks to cursor
        if let Some(i) = grabbed {
            // follow mouse
            points[i].pos = mouse;
            // "throw" on release by keeping velocity tied to mouse motion
            // (cap it so it doesn't explode)
            let cap = 900.0;
            points[i].vel = mouse_vel.clamp_length_max(cap);
        }

        // collisions (skip grabbed if you want it to be "infinite mass")
        let len = points.len();
        for a_i in 0..len {
            for b_i in (a_i + 1)..len {
                // optional: make grabbed particle act like a solid handle
                if grabbed == Some(a_i) || grabbed == Some(b_i) {
                    continue;
                }
                let (left, right) = points.split_at_mut(b_i);
                let a = &mut left[a_i];
                let b = &mut right[0];
                collide(a, b, r);
            }
        }

        // draw
        clear_background(BLACK);
        for (i, p) in points.iter().enumerate() {
            let col = if grabbed == Some(i) { YELLOW } else { WHITE };
            draw_circle(p.pos.x, p.pos.y, r, col);
        }
        // show pick radius
        // draw_circle_lines(mouse.x, mouse.y, pick_radius, 1.0, Color::new(1.0, 1.0, 1.0, 0.15));

        next_frame().await
    }
}
