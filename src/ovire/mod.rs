pub mod pravokotnik;
pub mod trikotnik;

use macroquad::prelude::*;
use crate::igralec::*;

#[derive(Debug)]
pub enum IzidTrka {
    None,
    Smrt,
    PristaniNaOviri(f32), // višina, na kateri smo pristali
}

pub trait Ovira {
    fn visina(&self) -> f32;
    fn sirina(&self) -> f32;
    fn narisi(&self, x: f32, y: f32, color: Color);
    fn preveri_trk(&self, o_x: f32, o_y: f32, p: &Igralec) -> IzidTrka;
}
