use macroquad::prelude::*;

use crate::zemljevid::*;

use super::igranje::Igra;
use super::stanje::IgralnoStanje;

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


pub fn posodobi(stanje: &mut IgralnoStanje, igra: &mut Igra, izbrana_stopnja: &mut bool) {
    if !*izbrana_stopnja {
        // uporabnik izbere stopnjo igre
        let miska_beginner = miska_pravokotni_gumb(0.0);
        if miska_beginner && is_mouse_button_pressed(MouseButton::Left) {
            *izbrana_stopnja = true;
            return;
        }
        
        let miska_advanced = miska_pravokotni_gumb(80.0);
        if miska_advanced && is_mouse_button_pressed(MouseButton::Left) {
            igra.stopnja = Stopnja::Advanced;
            *izbrana_stopnja = true;
            return;
        }
    }

    if *izbrana_stopnja {
        let miska_nad_gumbom = miska_trikotni_gumb();

        // Interakcija za zagon igre
        if (miska_nad_gumbom
            && is_mouse_button_pressed(MouseButton::Left))
            || is_key_pressed(KeyCode::Space)
        {
            igra.zemljevid = naredi_zemljevid(&igra.stopnja);
            *stanje = IgralnoStanje::Igra;
        }
    }
}


pub fn narisi(best_score: u32, izbrana_stopnja: bool) {
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

    if !izbrana_stopnja {
        // gumba za izbiro stopnje
        let x = screen_width() / 2.0 - 100.0;
        let y = screen_height() / 2.0;

        draw_rectangle(x, y, 200.0, 60.0, BLUE);

        let tekst = "BEGINNER";
        let tekst_sirina = measure_text(tekst, None, 25, 1.0).width;
        
        draw_text(
            tekst, 
            x + (200.0 - tekst_sirina) / 2.0, // tekst poravnan na sredini 
            y + 38.0,
            25.0, WHITE
        );

        draw_rectangle(x, y + 80.0, 200.0, 60.0, BLUE);

        let text = "ADVANCED";
        let text_sirina = measure_text(text, None, 25, 1.0).width;
        
        draw_text(
            text, 
            x + (200.0 - text_sirina) / 2.0,
            y + 118.0,
            25.0,
            WHITE
        );
    }

    if izbrana_stopnja {
        // --- TRIKOTNI "PLAY / RESUME" GUMB ---
        let sredisce_x = screen_width() / 2.0;
        let sredisce_y = screen_height() / 2.0 + 20.0;
        let polmer = 50.0; // Velikost trikotnika

        // Definiramo 3 oglišča trikotnika (obrnjeno v desno ▶)
        let v1 = Vec2::new(sredisce_x - polmer * 0.5, sredisce_y - polmer);
        let v2 = Vec2::new(sredisce_x - polmer * 0.5, sredisce_y + polmer);
        let v3 = Vec2::new(sredisce_x + polmer, sredisce_y);

        let miska_nad_gumbom = miska_trikotni_gumb();

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
}
