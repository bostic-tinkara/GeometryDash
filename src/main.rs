mod igralec;
mod ovire;
mod testi;
mod trki;
mod zemljevid;
mod igra;

use macroquad::prelude::*;
use igra::igranje::Igra;
use igra::stanje::IgralnoStanje;
use zemljevid::Stopnja;
use igra::{meni, pavza};

#[macroquad::main("Geometry Dash")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    // če želimo, da se generator naključnih števil spreminja

    let stopnja = Stopnja::Beginner;
    let mut igra = Igra::new(stopnja);
    let mut stanje = IgralnoStanje::Meni;

    loop {
        igra::ozadje::narisi_ozadje(&stanje);

        match stanje {
            IgralnoStanje::Meni => {
                meni::posodobi(&mut stanje);
                meni::narisi(igra.best_score);
            }

            IgralnoStanje::Pavza => {
                pavza::posodobi(&mut stanje, &mut igra);
                pavza::narisi(igra.score_accumulator, igra.best_score);
            }

            IgralnoStanje::Igra => {
                igra.posodobi(&mut stanje);
                igra.narisi();
            }
        }
            
        next_frame().await;
    }
}
