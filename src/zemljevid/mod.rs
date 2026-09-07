pub mod beginner;
pub mod advanced;

use macroquad::prelude::*;
use crate::{ovire::IzidTrka, igralec::Igralec};


/// Začetna hitrost premikanja ovir (enote na sličico pri ~60 FPS).
pub const ZACETNA_HITROST: f32 = 3.0;
/// Prvih toliko sekund igranja se hitrost sploh ne spreminja.
pub const ZAKASNITEV_POSPESKA: f32 = 40.0;
/// Po zakasnitvi hitrost zelo počasi raste za toliko na sekundo.
pub const POSPESEK_NA_SEKUNDO: f32 = 0.03;
/// Pri tej (res veliki) hitrosti se pospeševanje ustavi in hitrost postane konstantna.
pub const NAJVECJA_HITROST: f32 = 12.0;


#[derive(Clone, Copy, PartialEq)]
pub enum Stopnja {
    Beginner,
    Advanced,
}


pub trait Zemljevid {
    fn dodaj_oviro(&mut self);
    fn posodobi(&mut self, dt: f32);
    fn narisi(&self, tla_y: f32);
    fn preveri_trk(&self, trenutna_tla: f32, igralec: &Igralec) -> IzidTrka;
}


pub fn naredi_zemljevid(stopnja: &Stopnja) -> Box<dyn Zemljevid> {
    match stopnja {
        Stopnja::Beginner => Box::new(beginner::Beginner::new()),
        Stopnja::Advanced => Box::new(advanced::Advanced::new()),
    }
}

impl Stopnja {
    pub fn najvec_skokov(&self) -> u32 {
        match self {
            Stopnja::Beginner => 1,
            Stopnja::Advanced => 3,
        }
    }

    /// Ime stopnje za prikaz na gumbih.
    pub fn ime(&self) -> &'static str {
        match self {
            Stopnja::Beginner => "BEGINNER",
            Stopnja::Advanced => "ADVANCED",
        }
    }

    /// Naslednja stopnja v krogu (za preklop v pavzi).
    pub fn naslednja(&self) -> Stopnja {
        match self {
            Stopnja::Beginner => Stopnja::Advanced,
            Stopnja::Advanced => Stopnja::Beginner,
        }
    }
}
