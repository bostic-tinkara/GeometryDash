mod igralec;
mod ovire;
mod testi;
mod trki;
mod zemljevid;

use crate::zemljevid::beginner::Beginner;
use igralec::*;
use macroquad::prelude::*;
use ovire::*;
use std::time::Duration;
use zemljevid::*;

enum IgralnoStanje {
    Meni,
    Igra,
    Pavza,
}

#[macroquad::main("Geometry Dash")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    // če želimo, da se generator naključnih števil spreminja

    next_frame().await; // počakamo, da se naloži ekran

    let gravitacija = 0.35;
    let mut igralec = Igralec::new(40.0, screen_height());
    let mut zemljevid: Box<dyn Zemljevid> = Box::new(Beginner::new());
    let mut score_accumulator: f32 = 0.0;
    let mut best_score: u32 = 0;

    let mut dead = false;
    let mut cas_smrti = 0.0;

    let mut stanje = IgralnoStanje::Meni;

    loop {
        let bubble_gum = Color::new(1.00, 0.43, 0.76, 1.00);
        clear_background(bubble_gum);

        match stanje {
            IgralnoStanje::Meni => {
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

                // Zaznavanje premika miške nad trikotnikom (poenostavljen krog/kvadrat hit-box)
                let (miska_x, miska_y) = mouse_position();
                let miska_nad_gumbom = miska_x >= sredisce_x - polmer
                    && miska_x <= sredisce_x + polmer
                    && miska_y >= sredisce_y - polmer
                    && miska_y <= sredisce_y + polmer;

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

                // Interakcija za zagon igre
                if (miska_nad_gumbom && is_mouse_button_pressed(MouseButton::Left))
                    || is_key_pressed(KeyCode::Space)
                {
                    score_accumulator = 0.0;
                    zemljevid = Box::new(Beginner::new());
                    let osnovna_tla = screen_height() - 100.0;
                    igralec.y = osnovna_tla - igralec.stranica;
                    igralec.y_hitrost = 0.0;
                    igralec.rotacija = 0.0;
                    dead = false;

                    stanje = IgralnoStanje::Igra;
                }
            }

            IgralnoStanje::Pavza => {
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
                let resume_x = screen_width() / 2.0 - 60.0;
                let resume_y = screen_height() / 2.0 + 20.0;
                let polmer = 40.0;

                let v1 = Vec2::new(resume_x - polmer * 0.5, resume_y - polmer);
                let v2 = Vec2::new(resume_x - polmer * 0.5, resume_y + polmer);
                let v3 = Vec2::new(resume_x + polmer, resume_y);

                let miska_nad_resume = miska_x >= resume_x - polmer
                    && miska_x <= resume_x + polmer
                    && miska_y >= resume_y - polmer
                    && miska_y <= resume_y + polmer;

                let barva_resume = if miska_nad_resume { DARKBLUE } else { BLUE };
                draw_triangle(v1, v2, v3, barva_resume);
                draw_triangle_lines(v1, v2, v3, 4.0, WHITE);

                // 2. GUMB: RESTART (Kvadrat s tekstom "RESTART" ali "C") - Začne znova
                let restart_x = screen_width() / 2.0 + 60.0;
                let restart_y = screen_height() / 2.0 + 20.0;
                let restart_velikost = 60.0;
                let restart_kvadrat = Rect::new(
                    restart_x - restart_velikost / 2.0,
                    restart_y - restart_velikost / 2.0,
                    restart_velikost,
                    restart_velikost,
                );

                let miska_nad_restart = restart_kvadrat.contains(vec2(miska_x, miska_y));
                let barva_restart = if miska_nad_restart { DARKBLUE } else { BLUE };

                draw_rectangle(
                    restart_kvadrat.x,
                    restart_kvadrat.y,
                    restart_kvadrat.w,
                    restart_kvadrat.h,
                    barva_restart,
                );
                draw_rectangle_lines(
                    restart_kvadrat.x,
                    restart_kvadrat.y,
                    restart_kvadrat.w,
                    restart_kvadrat.h,
                    3.0,
                    WHITE,
                );

                // Simbol za ponovni zagon na gumbu
                let tekst_restart = "RESTART";
                let r_mere = measure_text(tekst_restart, None, 40, 1.0);
                draw_text(
                    tekst_restart,
                    restart_x - r_mere.width / 2.0,
                    restart_y + r_mere.height / 2.0 - 2.0,
                    40.0,
                    WHITE,
                );

                draw_text(&format!("Score: {}", score_accumulator as u32), 10.0, 30.0, 30.0, WHITE);
                draw_text(&format!("Best Score: {}", best_score), 10.0, 60.0, 30.0, WHITE);

                // Klik na RESUME -> Nadaljuje igro
                if (miska_nad_resume && is_mouse_button_pressed(MouseButton::Left))
                    || is_key_pressed(KeyCode::Space)
                {
                    stanje = IgralnoStanje::Igra;
                }

                // Klik na RESTART -> Ponastavi igro in začne znova
                if miska_nad_restart && is_mouse_button_pressed(MouseButton::Left) {
                    score_accumulator = 0.0;
                    zemljevid = Box::new(Beginner::new());
                    let osnovna_tla = screen_height() - 100.0;
                    igralec.y = osnovna_tla - igralec.stranica;
                    igralec.y_hitrost = 0.0;
                    igralec.rotacija = 0.0;
                    dead = false;

                    stanje = IgralnoStanje::Igra;
                }
            }

            IgralnoStanje::Igra => {
                //tla, kjer je igralec, če ni na nobeni oviri (lahko je tudi na oviri, ne samo na tleh)
                let osnovna_tla = screen_height() - 100.0;
                let current_score = score_accumulator as u32;

                // ob trku
                if dead {
                    let bubble_gum = Color::new(1.00, 0.43, 0.76, 1.00);
                    clear_background(bubble_gum);
                    draw_line(0.0, osnovna_tla, screen_width(), osnovna_tla, 2.0, WHITE);
                    zemljevid.narisi(osnovna_tla);

                    let sinus = (get_time() * 25.0).sin(); // ko igralec umre, utripa
                    if sinus > 0.0 {
                        igralec.narisi(BLUE);
                    }

                    if get_time() - cas_smrti > 1.0 {
                        // igra se ponovno začne
                        score_accumulator = 0.0;
                        zemljevid = Box::new(Beginner::new());
                        igralec.y = osnovna_tla - igralec.stranica;
                        igralec.y_hitrost = 0.0;
                        igralec.rotacija = 0.0;
                        dead = false;
                    }
                } else {
                    // set up za igro

                    zemljevid.posodobi();

                    let dt = get_frame_time();
                    score_accumulator += 10.0 * dt;

                    let mut trenutna_tla = osnovna_tla;
                    let mut na_oviri = false;

                    // GUMB ZA MENI / PAUZO (Zgornji desni kot - ikona ❚❚) ---
                    let meni_gumb_x = screen_width() - 60.0;
                    let meni_gumb_y = 10.0;
                    let meni_gumb_velikost = 40.0;

                    let (miska_x, miska_y) = mouse_position();
                    let miska_nad_meni_gumbom = miska_x >= meni_gumb_x
                        && miska_x <= meni_gumb_x + meni_gumb_velikost
                        && miska_y >= meni_gumb_y
                        && miska_y <= meni_gumb_y + meni_gumb_velikost;

                    // Če kliknemo gumb za Meni, preklopimo stanje
                    if miska_nad_meni_gumbom && is_mouse_button_pressed(MouseButton::Left) {
                        if current_score > best_score {
                            best_score = current_score;
                        }
                        stanje = IgralnoStanje::Pavza;
                    }

                    // (Preprečimo skok ob kliku na gumb) ---
                    let klik_za_skok = is_mouse_button_pressed(MouseButton::Left) && !miska_nad_meni_gumbom;
                    if is_key_down(KeyCode::Space) || klik_za_skok {
                        igralec.skoci();
                    }

                    // RISANJE "RESUME/PAUSE" GUMBA ---
                    let barva_meni_gumba = if miska_nad_meni_gumbom { DARKBLUE } else { BLUE };

                    // Kvadratna podlaga z belim robom
                    draw_rectangle(meni_gumb_x, meni_gumb_y, meni_gumb_velikost, meni_gumb_velikost, barva_meni_gumba);
                    draw_rectangle_lines(meni_gumb_x, meni_gumb_y, meni_gumb_velikost, meni_gumb_velikost, 2.5, WHITE);

                    // Narisemo dve beli navpični črtici (Pauza ikona: ❚❚)
                    let pas_sirina = 6.0;
                    let pas_visina = 20.0;
                    let odmerek_y = meni_gumb_y + 10.0;

                    draw_rectangle(meni_gumb_x + 11.0, odmerek_y, pas_sirina, pas_visina, WHITE);
                    draw_rectangle(meni_gumb_x + 23.0, odmerek_y, pas_sirina, pas_visina, WHITE);

                    if is_key_down(KeyCode::Space) {
                        igralec.skoci();
                    }

                    igralec.posodobi(gravitacija, trenutna_tla);

                    match zemljevid.preveri_trk(trenutna_tla, &igralec) {
                        IzidTrka::None => {}
                        IzidTrka::Smrt => {
                            dead = true;
                            cas_smrti = get_time();

                            if current_score > best_score {
                                best_score = current_score;
                            }
                        }
                        IzidTrka::PristaniNaOviri(visina) => {
                            na_oviri = true;
                            trenutna_tla = visina;

                            // igralec pristane zgoraj
                            igralec.y = trenutna_tla - igralec.stranica;
                            igralec.y_hitrost = 0.0;
                            igralec.rotacija = 0.0;
                        }
                    }

                    if !na_oviri {
                        trenutna_tla = osnovna_tla;
                    }

                    // Rotacija
                    let v_zraku = igralec.y < trenutna_tla - igralec.stranica - 0.1;
                    if v_zraku {
                        igralec.rotacija += 400.0 * dt;
                    } else {
                        let ciljna_poravnava = (igralec.rotacija / 90.0).round() * 90.0;
                        igralec.rotacija += (ciljna_poravnava - igralec.rotacija) * 0.3;
                    }

                    draw_line(0.0, osnovna_tla, screen_width(), osnovna_tla, 2.0, WHITE);
                    zemljevid.narisi(osnovna_tla);
                    igralec.narisi(BLUE);
                }

                draw_text(
                    &format!("Score: {}", current_score),
                    10.0,
                    30.0,
                    30.0,
                    WHITE,
                );

            }
        }

        next_frame().await;
    }
}
