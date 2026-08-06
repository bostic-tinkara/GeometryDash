pub mod beginner;

use macroquad::prelude::*;
use crate::{ovire::IzidTrka, igralec::Igralec};

pub trait Zemljevid {
    fn dodaj_oviro(&mut self);
    fn posodobi(&mut self);
    fn narisi(&self, tla_y: f32);
    fn preveri_trk(&self, trenutna_tla: f32, igralec: &Igralec) -> IzidTrka;
}
