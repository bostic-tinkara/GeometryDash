// te funkcije uporabljava v modulih igranje, meni in pavza

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

pub fn miska_trikotni_gumb() -> bool {
    // Zaznavanje premika miške nad trikotnikom (poenostavljen krog/kvadrat hit-box)
    let (miska_x, miska_y) = mouse_position();

    // --- TRIKOTNI "PLAY / RESUME" GUMB ---
    let sredisce_x = screen_width() / 2.0;
    let sredisce_y = screen_height() / 2.0 + 20.0;
    let polmer = 50.0; // Velikost trikotnika

    miska_x >= sredisce_x - polmer
        && miska_x <= sredisce_x + polmer
        && miska_y >= sredisce_y - polmer
        && miska_y <= sredisce_y + polmer
}

pub fn miska_pravokotni_gumb(zamik_y: f32) -> bool {
    let x = screen_width() / 2.0 - 100.0;
    let y = screen_height() / 2.0 + zamik_y;
    let gumb_sirina = 200.0;
    let gumb_visina = 60.0;

    let (miska_x, miska_y) = mouse_position();

    miska_x >= x
        && miska_x <= x + gumb_sirina
        && miska_y >= y
        && miska_y <= y + gumb_visina
}

pub fn resume_gumb() -> (Vec2, Vec2, Vec2, Rect) {
    let x = screen_width() / 2.0 - 60.0;
    let y = screen_height() / 2.0 + 20.0;
    let polmer = 40.0;

    let v1 = vec2(x - polmer * 0.5, y - polmer);
    let v2 = vec2(x - polmer * 0.5, y + polmer);
    let v3 = vec2(x + polmer, y);

    let hitbox = Rect::new(
        x - polmer,
        y - polmer,
        polmer * 2.0,
        polmer * 2.0,
    );

    (v1, v2, v3, hitbox)
}

pub fn restart_gumb() -> (f32, f32, f32, Rect) {
    let restart_x = screen_width() / 2.0 + 80.0;
    let restart_y = screen_height() / 2.0 + 20.0;
    let polmer = 18.0;

    let hitbox = Rect::new(
        restart_x - polmer - 10.0,
        restart_y - polmer - 10.0,
        (polmer + 10.0) * 2.0,
        (polmer + 10.0) * 2.0,
    );

    (restart_x, restart_y, polmer, hitbox)
}
