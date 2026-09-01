use macroquad::prelude::*;

pub fn meni_gumb() -> (f32, f32, f32) {
    (screen_width() - 60.0, 10.0, 40.0)
}

pub fn miska_nad_meni_gumbom() -> bool {
    let (x, y, velikost) = meni_gumb();
    let (miska_x, miska_y) = mouse_position();

    miska_x >= x
        && miska_x <= x + velikost
        && miska_y >= y
        && miska_y <= y + velikost
}
