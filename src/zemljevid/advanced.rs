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


#[derive(Clone, Copy, PartialEq)]
enum TipOvire {
    // pomaga pri sestavljanju stolpov
    Kvadrat,
    Trikotnik,
}

pub struct PostavitevOvir {
    // posamezna ovira
    pub y_od_tal: f32,
    pub tip: TipOvire,
    pub ovira: Box<dyn Ovira>,
}

pub struct Stolp {
    // stolp iz ovir
    pub x: f32,
    pub ovire: Vec<PostavitevOvir>,
    pub razdalja: f32,
}

pub struct Advanced {
    // poligon je sestavljen iz stolpov
    pub poligon: Vec<Stolp>,
    pub hitrost: f32,
}

impl Advanced {
    pub fn new() -> Self {
        Self {
            poligon: Vec::new(),
            hitrost: 3.0,
        }
    }

    fn naredi_oviro(tip: TipOvire, sirina: f32) -> Box<dyn Ovira> {
        match tip {
            TipOvire::Kvadrat => {
                Box::new(Pravokotnik::kvadrat(sirina))
            }
            TipOvire::Trikotnik => { 
                Box::new(Trikotnik::enakostranicni(sirina))
            }
        }
    }

    fn dodaj_v_stolp(
        stolp: &mut Stolp,
        tip: TipOvire,
        y_od_tal: f32,
        sirina: f32,
    ) {
        // novih ovir ne moremo postaviti na trikotnik, zato preverimo
        if let Some(zadnja) = stolp.ovire.last() {
            if zadnja.tip == TipOvire::Trikotnik {
                return;
            }
        }

        // sicer v stolp dodamo novo oviro
        let ovira = Self::naredi_oviro(tip, sirina);
        
        stolp.ovire.push(PostavitevOvir {
            y_od_tal,
            tip,
            ovira,
        });
    }

    fn naredi_stolp(&mut self) {
        let mut stolp = Stolp {
            x: screen_width(),
            ovire: Vec::new(),
            razdalja: rand::gen_range(150.0, 300.0),
        };
        
        let sirina = rand::gen_range(30.0, 50.0);
        
        if rand::gen_range(0, 5) == 0 {
            // petina ovir bo posameznih
            let tip = 
                if rand::gen_range(0, 2) == 0 {
                    TipOvire::Kvadrat
                } else {
                    TipOvire::Trikotnik
                };

            Self::dodaj_v_stolp(&mut stolp, tip, 0.0, sirina);
        
        } else {
            // stolp z 2-4 ovirami
            let stevilo_ovir = rand::gen_range(2, 5);
            let mut y_od_tal = 0.0;

            // spodnja ovira je vedno kvadrat
            Self::dodaj_v_stolp(
                &mut stolp,
                TipOvire::Kvadrat,
                y_od_tal,
                sirina,
            );

            y_od_tal += sirina;

            for _ in 1..stevilo_ovir {
                // preverimo, ali je zadnja ovira trikotnik
                // ce je, ne dodamo vec nicesar
                if stolp.ovire.last().unwrap().tip == TipOvire::Trikotnik {
                    break;
                }

                let tip = if rand::gen_range(0, 2) == 0 {
                    TipOvire::Kvadrat
                } else {
                    TipOvire::Trikotnik
                };

                Self::dodaj_v_stolp(
                    &mut stolp,
                    tip,
                    y_od_tal,
                    sirina,
                );

                y_od_tal += sirina;
            }
        }

        self.poligon.push(stolp);
    }
}

impl Zemljevid for Advanced {
    fn dodaj_oviro(&mut self) {
        self.naredi_stolp();
    }


    fn posodobi(&mut self) {
        for stolp in &mut self.poligon { // gibanje ovir/stolpov
            stolp.x -= self.hitrost;
        }

        // odstrani ovire/stolpe, ko zapustijo ekran
        self.poligon.retain(|stolp| {
            if let Some(prva_ovira) = stolp.ovire.first() {
                stolp.x > -prva_ovira.ovira.sirina()
            } else {
                false
            }
        });

        if self.poligon.is_empty() {
            // dodamo novo oviro/stolp, ce je poligon prazen
            self.dodaj_oviro();
        } else if let Some(zadnji_stolp) = self.poligon.last() {
            if let Some(prva_ovira) = zadnji_stolp.ovire.first() {
                // dodamo novo oviro/stolp, ce je desni rob zadnje ovire dovolj dalec
                let desni_rob = zadnji_stolp.x + prva_ovira.ovira.sirina();

                if desni_rob < screen_width() - zadnji_stolp.razdalja {
                    self.dodaj_oviro();
                }
            }
        }
    }


    fn narisi(&self, tla_y: f32) {
        for stolp in &self.poligon {
            for postavitev in &stolp.ovire {
                let x = stolp.x;
                let y = tla_y - postavitev.y_od_tal;
                let desni_rob = x + postavitev.ovira.sirina();

                if desni_rob > 0.0 && x < screen_width() { 
                    // samo če je ovira blizu ekrana, jo narišemo
                    postavitev.ovira.narisi(x, y, DARKBLUE);
                }
            }
        }
    }


    fn preveri_trk(&self, trenutna_tla: f32, igralec: &Igralec) -> IzidTrka {
        for stolp in &self.poligon {
            for postavitev in &stolp.ovire {
                let y = trenutna_tla - postavitev.y_od_tal;

                match postavitev.ovira.preveri_trk(
                    stolp.x,
                    y,
                    igralec,
                ) {
                    IzidTrka::None => {},
                    IzidTrka::Smrt => return IzidTrka::Smrt,
                    IzidTrka::PristaniNaOviri(visina) => {
                        return IzidTrka::PristaniNaOviri(visina);
                    }
                }
            }
        }

        IzidTrka::None
    }
}
