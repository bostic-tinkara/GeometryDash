mod ovire;
mod player;
mod testi;
mod zemljevid;

use macroquad::prelude::*;
use ovire::*;
use player::*;
use zemljevid::*;

#[macroquad::main("Geometry Dash Lite")]
async fn main() {
    next_frame().await; // počakamo, da se naloži ekran

    let gravitacija = 0.35;
    let mut igralec = Player::new(40.0);
    let mut zemljevid = Zemljevid::new(Stopnja::Beginner);

    loop {
        let bubble_gum = Color::new(1.00, 0.43, 0.76, 1.00);
        clear_background(bubble_gum);

        let osnovna_tla = screen_height() - 100.0; //tla kjer je igralec, če ni na nobeni ovire
        let mut trenutna_tla = osnovna_tla; //lahko je tudi na oviri ne samo na tleh
        let mut na_oviri = false;

        zemljevid.posodobi();

        for (o_x, ovira) in &zemljevid.poligon {
            match ovira.preveri_trk(*o_x, trenutna_tla, &igralec) {
                IzidTrka::None => {}
                IzidTrka::Smrt => {
                    println!("Umrl si!");
                    return;
                }
                IzidTrka::PristaniNaOviri(visina) => {
                    na_oviri = true;
                    trenutna_tla = visina;
                }
            }
            
        }

        if !na_oviri {
            trenutna_tla = osnovna_tla;
        }

        if is_key_down(KeyCode::Space) {
            igralec.skoci();
        }
         
        igralec.posodobi(gravitacija, trenutna_tla);

        draw_line(0.0, osnovna_tla, screen_width(), osnovna_tla, 2.0, WHITE);
        zemljevid.narisi(osnovna_tla);
        igralec.narisi(BLUE);
 
        next_frame().await
    }
}
