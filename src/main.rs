mod igralec;
mod ovire;
#[cfg(test)]
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

    // Glasba: ozadje med igranjem, glasba v meniju in kratek jingle ob smrti
    macroquad::file::set_pc_assets_folder("assets");
    let glasba = macroquad::audio::load_sound("glasba_ozadje.wav").await.ok();
    let glasba_meni = macroquad::audio::load_sound("glasba_meni.wav").await.ok();
    let smrt_jingle = macroquad::audio::load_sound("smrt_jingle.wav").await.ok();

    let mut igra = Igra::new(Stopnja::Beginner);
    igra.glasba = glasba;
    igra.glasba_meni = glasba_meni;
    igra.smrt_jingle = smrt_jingle;
    let mut izbrana_stopnja = false;
    let mut stanje = IgralnoStanje::Meni;

    loop {
        igra::posodobi(&mut stanje, &mut igra, &mut izbrana_stopnja);
        igra::narisi(&stanje, &igra, izbrana_stopnja);

        next_frame().await;
    }
}
