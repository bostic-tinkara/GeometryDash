mod ovire;
mod player;
mod zemljevid;
mod testi;

use macroquad::prelude::*;
use ovire::*;
use player::*;

#[macroquad::main("Geometry Dash Lite")]
async fn main() {
    loop {

        let bubble_gum = Color::new(1.00, 0.43, 0.76, 1.00);
        clear_background(bubble_gum);

        draw_text("Hello Macroquad", 20.0, 30.0, 30.0, WHITE);
        draw_rectangle(screen_width() / 2.0 - 60.0, screen_height() / 2.0-30.0, 120.0, 60.0, SKYBLUE);

        next_frame().await
    }
}
