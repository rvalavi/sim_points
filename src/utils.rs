use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
    pub vel: Vec2,
}


pub fn rand_vel() -> Vec2 {
    let angle = rand::gen_range(0.0, std::f32::consts::TAU);
    let speed = rand::gen_range(40.0, 120.0);
    vec2(angle.cos(), angle.sin()) * speed
}


pub fn reset_points(points: &mut Vec<Point>, n: usize, r: f32, w: f32, h: f32) {
    points.clear();
    points.reserve(n);
    for _ in 0..n {
        points.push(Point {
            pos: vec2(rand::gen_range(r, w - r), rand::gen_range(r, h - r)),
            vel: rand_vel(),
        });
    }
}


pub fn bounce_walls(p: &mut Point, w: f32, h: f32, r: f32) {
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


pub fn collide(a: &mut Point, b: &mut Point, r: f32) {
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


pub fn nearest_point(points: &[Point], mouse: Vec2, max_dist: f32) -> Option<usize> {
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

