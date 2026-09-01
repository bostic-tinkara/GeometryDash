use macroquad::prelude::*;

use super::stanje::IgralnoStanje;

pub fn miska_nad_gumbom() -> bool {
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

pub fn posodobi(stanje: &mut IgralnoStanje) {
    let miska_nad_gumbom = miska_nad_gumbom();

    // Interakcija za zagon igre
    if (miska_nad_gumbom 
        && is_mouse_button_pressed(MouseButton::Left))
        || is_key_pressed(KeyCode::Space)
    {
        // score_accumulator = 0.0;
        // zemljevid = Box::new(Beginner::new());
        // let osnovna_tla = screen_height() - 100.0;
        // igralec.y = osnovna_tla - igralec.stranica;
        // igralec.y_hitrost = 0.0;
        // igralec.rotacija = 0.0;
        // dead = false;
        *stanje = IgralnoStanje::Igra;
    }
}


pub fn narisi(best_score: u32) {
    let naslov = "GEOMETRY DASH";
    let naslov_velikost = 50.0;
    let naslov_mere = measure_text(naslov, None, naslov_velikost as u16, 1.0);

    draw_text(
        naslov,
        screen_width() / 2.0 - naslov_mere.width / 2.0,
        screen_height() / 3.0,
        naslov_velikost,
        WHITE,
    );

    // --- TRIKOTNI "PLAY / RESUME" GUMB ---
    let sredisce_x = screen_width() / 2.0;
    let sredisce_y = screen_height() / 2.0 + 20.0;
    let polmer = 50.0; // Velikost trikotnika

    // Definiramo 3 oglišča trikotnika (obrnjeno v desno ▶)
    let v1 = Vec2::new(sredisce_x - polmer * 0.5, sredisce_y - polmer);
    let v2 = Vec2::new(sredisce_x - polmer * 0.5, sredisce_y + polmer);
    let v3 = Vec2::new(sredisce_x + polmer, sredisce_y);

    let miska_nad_gumbom = miska_nad_gumbom();

    // Barva se ob prehodu miške posvetli ali potemni
    let barva_gumba = if miska_nad_gumbom { DARKBLUE } else { BLUE };

    // 1. Polnjenje trikotnika
    draw_triangle(v1, v2, v3, barva_gumba);

    // 2. Beli rob okoli trikotnika za Resume izgled
    draw_triangle_lines(v1, v2, v3, 4.0, WHITE);

    // Prikaz najboljšega rezultata v meniju
    draw_text(
        &format!("Best Score: {}", best_score),
        10.0,
        30.0,
        30.0,
        WHITE,
    );
}
