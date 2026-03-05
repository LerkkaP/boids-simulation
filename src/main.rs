use macroquad::prelude::*;

struct Boid {
    v1: Vec2,
    v2: Vec2,
    v3: Vec2,
    rotation: f32,
}

impl Boid {
    fn get_center(&self) -> (f32, f32) {
        let x = (self.v1.x + self.v2.x + self.v3.x) / 3.0;
        let y = (self.v1.y + self.v2.y + self.v3.y) / 3.0;
        (x, y)
    }

    fn angle_to_mouse(&self, mouse_x: f32, mouse_y: f32) -> f32 {
        let (x, y) = self.get_center();

        // center to mouse
        let dx_mouse = mouse_x - x;
        let dy_mouse = mouse_y - y;

        // center to tip
        let dx_tip_cen = self.v2.x - x;
        let dy_tip_cen = self.v2.y - y;

        // in radians
        let mouse_angle = dy_mouse.atan2(dx_mouse);
        let tip_angle = dy_tip_cen.atan2(dx_tip_cen);

        // rotation needed
        mouse_angle - tip_angle
    }

    fn rotate(&mut self, angle: f32) -> () {
        let (x_center, y_center) = self.get_center();

        for v in [&mut self.v1, &mut self.v2, &mut self.v3] {
            // translate the rotation center to the origin
            let x = v.x - x_center;
            let y = v.y - y_center;

            // calculate new coordinates
            let x_new = x * angle.cos() - y * angle.sin();
            let y_new = x * angle.sin() + y * angle.cos();

            // translate back to original center
            v.x = x_new + x_center;
            v.y = y_new + y_center;
        }
    }

    fn move_boid(&mut self, mouse_x: f32, mouse_y: f32) -> () {
        let delta = get_frame_time();
        let speed = 150.0;

        let (center_x, center_y) = self.get_center();

        let dx = mouse_x - center_x;
        let dy = mouse_y - center_y;

        let length = (dx*dx + dy*dy).sqrt();

        let dir_x = dx / length;
        let dir_y = dy / length;

        let velocity_x = dir_x * speed * delta;
        let velocity_y = dir_y * speed * delta;

        self.v1.x += velocity_x;
        self.v1.y += velocity_y;

        self.v2.x += velocity_x;
        self.v2.y += velocity_y;

        self.v3.x += velocity_x;
        self.v3.y += velocity_y;
    }
}

#[macroquad::main("Boid simulation")]
async fn main() {

    let (screen_center_x, screen_center_y) = (screen_width() / 2.0, screen_height() / 2.0);
    let size = 10.0;
    let multiplier = 3.0;
    let mut boid = Boid {
        v1: vec2(screen_center_x - size, screen_center_y + size * multiplier),
        v2: vec2(screen_center_x, screen_center_y),
        v3: vec2(screen_center_x + size, screen_center_y + size * multiplier),
        rotation: 0.0,
    };
    let mut trace: Vec<(f32, f32)> = Vec::new();

    loop {
        clear_background(BLACK);
        let (mouse_x, mouse_y) = mouse_position();
        let boid_center = boid.get_center();
        let delta = boid.angle_to_mouse(mouse_x, mouse_y);
        boid.rotate(delta);
        boid.rotation += delta;
        boid.move_boid(mouse_x, mouse_y);
        draw_triangle(boid.v1, boid.v2, boid.v3, WHITE);
        for pos in &trace {
            draw_circle(pos.0, pos.1, 1.0, BLUE);
        }
        trace.push((boid_center.0, boid.v1.y));
        next_frame().await
    }
}