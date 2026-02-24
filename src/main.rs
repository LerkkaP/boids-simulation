use macroquad::prelude::*;

struct Boid {
    v1: Vec2,
    v2: Vec2,
    v3: Vec2,
}

impl Boid {
    fn get_center(&self) -> (f32, f32) {
        let x = (self.v1.x + self.v2.x + self.v3.x) / 3.0;
        let y = (self.v1.y + self.v2.y + self.v3.y) / 3.0;
        (x, y)
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
    };

    loop {
        clear_background(BLACK);
        
        let (mouse_x, mouse_y) = mouse_position();
        draw_triangle(boid.v1, boid.v2, boid.v3, WHITE);
        boid.v2 = Vec2{x: mouse_x, y: mouse_y};
        
        next_frame().await
    }
}