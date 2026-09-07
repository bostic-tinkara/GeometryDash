pub mod stanje;
pub mod ozadje;
pub mod meni;
pub mod pavza;
pub mod igranje;
pub mod gumbi;

use crate::igra::{igranje::Igra, stanje::IgralnoStanje};


pub fn posodobi(
    stanje: &mut IgralnoStanje,
    igra: &mut Igra,
    izbrana_stopnja: &mut bool,
) {
    match stanje {
        IgralnoStanje::Meni =>
            meni::posodobi(stanje, igra, izbrana_stopnja),

        IgralnoStanje::Pavza =>
            pavza::posodobi(stanje, igra),

        IgralnoStanje::Igra =>
            igra.posodobi(stanje),
    }

    // uskladimo glasbo (meni / igra / jingle ob smrti) s trenutnim stanjem
    igra.posodobi_glasbo(stanje);
}


pub fn narisi(
    stanje: &IgralnoStanje,
    igra: &Igra,
    izbrana_stopnja: bool,
) {
    ozadje::narisi_ozadje(stanje, igra.dead);

    match stanje {
        IgralnoStanje::Meni => 
            meni::narisi(igra.best_score, izbrana_stopnja),

        IgralnoStanje::Pavza =>
            pavza::narisi(igra.score_accumulator, igra.best_score),

        IgralnoStanje::Igra => 
            igra.narisi(),
    }
}
