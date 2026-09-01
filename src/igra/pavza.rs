use macroquad::prelude::*;

use crate::igra::igranje::*;

use super::stanje::IgralnoStanje;

fn resume_gumb() -> (Vec2, Vec2, Vec2, Rect) {
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


fn restart_gumb() -> (f32, f32, f32, Rect) {
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


pub fn narisi(score_accumulator: f32, best_score: u32) {
    // Naslov
    let naslov = "RESUME";
    let naslov_velikost = 40.0;
    let naslov_mere = measure_text(naslov, None, naslov_velikost as u16, 1.0);
    draw_text(
        naslov,
        screen_width() / 2.0 - naslov_mere.width / 2.0,
        screen_height() / 3.0,
        naslov_velikost,
        WHITE,
    );

    let (miska_x, miska_y) = mouse_position();
    
    // 1. GUMB: RESUME (Trikotnik ▶) - Nadaljuje igro
    let (v1, v2, v3, resume_hitbox) = resume_gumb();
    
    let miska_nad_resume = resume_hitbox.contains(vec2(miska_x, miska_y));
    let barva_resume = if miska_nad_resume { DARKBLUE } else { BLUE };
    
    draw_triangle(v1, v2, v3, barva_resume);
    draw_triangle_lines(v1, v2, v3, 4.0, WHITE);

    // 2. GUMB: Pravilna okrogla krožna puščica (RESTART)
    let (restart_x, restart_y, polmer, restart_hitbox) = restart_gumb();

    let miska_nad_restart = restart_hitbox.contains(vec2(miska_x, miska_y));
    let barva = if miska_nad_restart { GREEN } else { WHITE };

    // 1. Risanje ukrivljenega krožnega loka točko po točki
    let stevi_tock = 20;
    let zacetni_kot = 45.0 * std::f32::consts::PI / 180.0;
    let koncni_kot = 320.0 * std::f32::consts::PI / 180.0;
    let korak = (koncni_kot - zacetni_kot) / (stevi_tock as f32);

    for i in 0..stevi_tock {
        let k1 = zacetni_kot + (i as f32) * korak;
        let k2 = zacetni_kot + ((i + 1) as f32) * korak;

        let p1 = vec2(restart_x + polmer * k1.cos(), restart_y - polmer * k1.sin());
        let p2 = vec2(restart_x + polmer * k2.cos(), restart_y - polmer * k2.sin());

        draw_line(p1.x, p1.y, p2.x, p2.y, 3.5, barva);
    }

    // 2. Majhna trikotna konica na vrhu loka
    let konica_x = restart_x + polmer * zacetni_kot.cos();
    let konica_y = restart_y - polmer * zacetni_kot.sin();

    let t1 = vec2(konica_x - 3.0, konica_y - 7.0);
    let t2 = vec2(konica_x + 7.0, konica_y + 1.0);
    let t3 = vec2(konica_x - 2.0, konica_y + 5.0);

    draw_triangle(t1, t2, t3, barva);

    draw_text(&format!("Score: {}", score_accumulator as u32), 10.0, 30.0, 30.0, WHITE);
    draw_text(&format!("Best Score: {}", best_score), 10.0, 60.0, 30.0, WHITE);
}


pub fn posodobi(stanje: &mut IgralnoStanje, igra: &mut Igra) {
    let (_, _, _, resume_hitbox) = resume_gumb();
    let (miska_x, miska_y) = mouse_position();
    let miska_nad_resume = resume_hitbox.contains(vec2(miska_x, miska_y));
    
    // Klik na RESUME -> Nadaljuje igro
    if (miska_nad_resume && is_mouse_button_pressed(MouseButton::Left))
        || is_key_pressed(KeyCode::Space)
    {
        *stanje = IgralnoStanje::Igra;
    }

    let (_, _, _, restart_hitbox) = restart_gumb();
    let (miska_x, miska_y) = mouse_position();
    let miska_nad_restart = restart_hitbox.contains(vec2(miska_x, miska_y));
    
    // Klik na RESTART -> Ponastavi igro in začne znova
    if miska_nad_restart && is_mouse_button_pressed(MouseButton::Left) {
        igra.ponovno_zazeni();
        *stanje = IgralnoStanje::Igra;
    }
}
