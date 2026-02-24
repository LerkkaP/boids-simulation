use macroquad::prelude::*;



#[macroquad::main("Ball")]
async fn main() {

    let screen_center = (screen_width() / 2.0, screen_height() / 2.0);

    loop {
        clear_background(BLACK);
        
        let (mouse_x, mouse_y) = (mouse_position().0, mouse_position().1);
        println!("Mouse x-position: {}", mouse_x);
        println!("Mouse y-position: {}", mouse_y);

        next_frame().await
    }
}