pub mod beginner;
pub mod advanced;

use macroquad::prelude::*;
use crate::{ovire::IzidTrka, igralec::Igralec};


#[derive(Clone, Copy, PartialEq)]
pub enum Stopnja {
    Beginner,
    Advanced,
}


pub trait Zemljevid {
    fn dodaj_oviro(&mut self);
    fn posodobi(&mut self);
    fn narisi(&self, tla_y: f32);
    fn preveri_trk(&self, trenutna_tla: f32, igralec: &Igralec) -> IzidTrka;
}


pub fn naredi_zemljevid(stopnja: &Stopnja) -> Box<dyn Zemljevid> {
    match stopnja {
        Stopnja::Beginner => Box::new(beginner::Beginner::new()),
        Stopnja::Advanced => Box::new(advanced::Advanced::new()),
    }
}
// let zemljevid = ustvari_zemljevid(&stopnja);