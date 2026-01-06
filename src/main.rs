use macroquad::prelude::*;
use std::env;
// local mods
mod utils;
use utils::*;


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
