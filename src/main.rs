mod ovire;
mod igralec;
mod testi;
mod zemljevid;

use macroquad::prelude::*;
use ovire::*;
use igralec::*;
use std::time::Duration;
use zemljevid::*;
use crate::zemljevid::beginner::Beginner;


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

    loop {

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

            if get_time() - cas_smrti > 1.0 { // igra se ponovno začne
                score_accumulator = 0.0;
                zemljevid = Box::new(Beginner::new());
                igralec.y = osnovna_tla - igralec.stranica;
                igralec.y_hitrost = 0.0;
                igralec.rotacija = 0.0;
                dead = false;
            }

        }

        else { // set up za igro

        let bubble_gum = Color::new(1.00, 0.43, 0.76, 1.00);
        clear_background(bubble_gum);

        let mut trenutna_tla = osnovna_tla;
        let mut na_oviri = false;

        zemljevid.posodobi();

        let dt = get_frame_time();
        score_accumulator += 10.0 * dt;

        match zemljevid.preveri_trk(trenutna_tla, &igralec) {
            IzidTrka::None => {},
            IzidTrka::Smrt => {
                dead = true;
                cas_smrti = get_time();
            
                if current_score > best_score {
                    best_score = current_score;
                }
            },
            IzidTrka::PristaniNaOviri(visina) => {
                na_oviri = true;
                trenutna_tla = visina;
                igralec.rotacija = 0.0;
            },
        }

        if !na_oviri {
            trenutna_tla = osnovna_tla;
        }

        if is_key_down(KeyCode::Space) {
            igralec.skoci();
        }

        igralec.posodobi(gravitacija, trenutna_tla);

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

        draw_text(
            &format!("Best Score: {}", best_score),
            10.0,
            60.0,
            30.0,
            WHITE,
        );

        next_frame().await
    }

}
