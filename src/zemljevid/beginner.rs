use macroquad::prelude::*;

use crate::{ 
    ovire::{
        pravokotnik::Pravokotnik, 
        trikotnik::Trikotnik, 
        IzidTrka,
        Ovira,
    },
    igralec::Igralec,
};

use super::Zemljevid;

pub struct PostavitevOvir {
    pub x: f32,
    pub ovira: Box<dyn Ovira>,
    pub razdalja: f32,
}

pub struct Beginner {
    pub poligon: Vec<PostavitevOvir>,
    pub hitrost: f32,
}

impl Beginner {
    pub fn new() -> Self {
        Self {
            poligon: Vec::new(),
            hitrost: 3.0,
        }
    }
}

impl Zemljevid for Beginner {
    fn dodaj_oviro(&mut self) {
        let razdalja = rand::gen_range(150.0, 300.0);

        if rand::gen_range(0, 2) == 0 {
            // Kvadrat
            let sirina = rand::gen_range(20.0, 60.0);
            self.poligon.push(PostavitevOvir {
                x: screen_width(),
                ovira: Box::new(Pravokotnik::kvadrat(sirina)),
                razdalja,
            });
        } else {
            // Enakostranicni trikotnik
            let sirina = rand::gen_range(30.0, 50.0);
            self.poligon.push(PostavitevOvir {
                x: screen_width(),
                ovira: Box::new(Trikotnik::enakostranicni(sirina)),
                razdalja,
            });
        }
    }


    fn posodobi(&mut self) {
        for postavitev in &mut self.poligon { // gibanje ovir
            postavitev.x -= self.hitrost;
        }

        // odstrani ovire, ko zapustijo ekran
        self.poligon.retain(|postavitev| postavitev.x > -postavitev.ovira.sirina());

        if self.poligon.is_empty() {
            // dodamo novo oviro, ce je poligon prazen
            self.dodaj_oviro();
        } else if let Some(zadnja_ovira) = self.poligon.last() {
            // dodamo novo oviro, ce je desni rob zadnje ovire dovolj dalec
            let desni_rob = zadnja_ovira.x + zadnja_ovira.ovira.sirina();

            if desni_rob < screen_width() - zadnja_ovira.razdalja {
                self.dodaj_oviro();
            }
        }
    }


    fn narisi(&self, tla_y: f32) {
        for postavitev in &self.poligon {
            let x = postavitev.x;
            let desni_rob = x + postavitev.ovira.sirina();

            if desni_rob > 0.0 && x < screen_width() { // samo če je ovira blizu ekrana, jo narišemo
                postavitev.ovira.narisi(x, tla_y, DARKBLUE);
            }
        }
    }


    fn preveri_trk(&self, trenutna_tla: f32, igralec: &Igralec) -> IzidTrka {
        for postavitev in &self.poligon {
            match postavitev.ovira.preveri_trk(
                postavitev.x,
                trenutna_tla,
                igralec,
            ) {
                IzidTrka::None => {},
                IzidTrka::Smrt => return IzidTrka::Smrt,
                IzidTrka::PristaniNaOviri(visina) => {
                    return IzidTrka::PristaniNaOviri(visina);
                }
            }
        }

        IzidTrka::None
    }
}
