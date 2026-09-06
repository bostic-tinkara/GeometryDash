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


#[macroquad::main("Geometry Dash")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    // če želimo, da se generator naključnih števil spreminja

    let mut igra = Igra::new(Stopnja::Beginner);
    let mut izbrana_stopnja = false;
    let mut stanje = IgralnoStanje::Meni;

    loop {
        igra::posodobi(&mut stanje, &mut igra, &mut izbrana_stopnja);
        igra::narisi(&stanje, &igra, izbrana_stopnja);

        next_frame().await;
    }
}
