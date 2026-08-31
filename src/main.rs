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

// Funkcija za risanje ozadja z bubble gum temo in dinamiko
fn narisi_ozadje(stanje: &IgralnoStanje) {
    // Osnovne barve palete
    let bubble_gum = Color::from_rgba(255, 110, 199, 255);
    let temna_magenta = Color::from_rgba(180, 50, 140, 255);
    let crte_mreze = Color::from_rgba(255, 255, 255, 40); // Rahlo prosojna bela

    clear_background(bubble_gum);

    let sirina = screen_width();
    let visina = screen_height();
    let tla_y = visina - 100.0;

    // 1. Risanje premikajoče se ozadne mreže za občutek globine
    let velikost_mreze = 40.0;
    let odmik_x = (get_time() * 30.0) as f32 % velikost_mreze;

    // Navpične črte
    let mut x = -odmik_x;
    while x < sirina {
        draw_line(x, 0.0, x, tla_y, 1.0, crte_mreze);
        x += velikost_mreze;
    }

    // Vodoravne črte
    let mut y = 0.0;
    while y < tla_y {
        draw_line(0.0, y, sirina, y, 1.0, crte_mreze);
        y += velikost_mreze;
    }

    // 2. Tla z dvobarvnim vzorcem
    draw_rectangle(0.0, tla_y, sirina, 100.0, temna_magenta);
    draw_line(0.0, tla_y, sirina, tla_y, 4.0, WHITE); // Svetleč zgornji rob tal

    // 3. Dodatni dekorativni gradientni sijaj na začetni strani
    if let IgralnoStanje::Meni = stanje {
        let cas = (get_time() * 2.0).sin() as f32 * 5.0;
        draw_circle(sirina / 2.0, visina / 3.0 - 10.0, 180.0 + cas, Color::from_rgba(255, 255, 255, 20));
    }
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
        narisi_ozadje(&stanje);

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

                // 2. GUMB: Pravilna okrogla krožna puščica (RESTART)
                let restart_x = screen_width() / 2.0 + 80.0;
                let restart_y = screen_height() / 2.0 + 20.0;
                let polmer = 18.0;

                let restart_hitbox = Rect::new(
                    restart_x - polmer - 10.0,
                    restart_y - polmer - 10.0,
                    (polmer + 10.0) * 2.0,
                    (polmer + 10.0) * 2.0,
                );

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
