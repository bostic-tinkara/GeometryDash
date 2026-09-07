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

use super::{Zemljevid, ZACETNA_HITROST, ZAKASNITEV_POSPESKA, POSPESEK_NA_SEKUNDO, NAJVECJA_HITROST};


#[derive(Clone, Copy, PartialEq)]
pub enum TipOvire {
    // pomaga pri sestavljanju stolpov
    Pravokotnik,
    Trikotnik,
}

pub struct PostavitevOvir {
    // posamezna ovira
    pub y_od_tal: f32, // na kateri višini je ovira
    pub x_odmik: f32, //horizontalna razdalja med ovirami znotraj stolpa
    pub tip: TipOvire,
    pub ovira: Box<dyn Ovira>,
}

pub struct Stolp {
    // stolp iz ovir
    pub x: f32,
    pub ovire: Vec<PostavitevOvir>,
    pub razdalja: f32, // razdalja do naslednjega stolpa
}

pub struct Advanced {
    // poligon je sestavljen iz stolpov
    pub poligon: Vec<Stolp>,
    pub hitrost: f32,
    pub cas: f32, // koliko sekund se že igra (za postopno pospeševanje)
}

impl Advanced {
    pub fn new() -> Self {
        Self {
            poligon: Vec::new(),
            hitrost: ZACETNA_HITROST,
            cas: 0.0,
        }
    }

    fn naredi_oviro(
        tip: TipOvire,
        visina:f32,
        sirina: f32,
    ) -> Box<dyn Ovira> {
        match tip {
            TipOvire::Pravokotnik => {
                Box::new(Pravokotnik::new(visina, sirina))
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
        x_odmik: f32,
        visina: f32,
        sirina: f32,
    ) {
        let ovira = Self::naredi_oviro(tip, visina, sirina);
        
        stolp.ovire.push(PostavitevOvir {
            y_od_tal,
            x_odmik,
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
                
        let zreb = rand::gen_range(0, 4);

        if zreb == 0 {
            // petina ovir bo posameznih oz. zaporednih (ena zraven druge)
            let stevilo = rand::gen_range(1, 4);
            let sirina = rand::gen_range(30.0, 50.0);
            let tip = 
                if rand::gen_range(0, 2) == 0 {
                    TipOvire::Pravokotnik
                } else {
                    TipOvire::Trikotnik
                };
            for i in 0..stevilo {
                Self::dodaj_v_stolp(
                    &mut stolp,
                    tip,
                    0.0,
                    sirina * i as f32,
                    sirina,
                    sirina,
                );
            }

        } else if zreb == 1 {
            // stopnice (pravokotniki po diagonali)
            let stevilo = rand::gen_range(2, 5);
            let visina = 30.0;
            let sirina = 80.0;
            for i in 0..stevilo {
                Self::dodaj_v_stolp(
                    &mut stolp,
                    TipOvire::Pravokotnik,
                    visina * i as f32,
                    sirina * i as f32,
                    visina,
                    sirina,
                );
            }
        
        } else {
            // stolp z 2-4 ovirami
            let stevilo_ovir = rand::gen_range(2, 5);
            let sirina = rand::gen_range(30.0, 50.0);
            let mut visina_y = 0.0;

            // spodnja ovira je vedno kvadrat
            Self::dodaj_v_stolp(
                &mut stolp,
                TipOvire::Pravokotnik,
                visina_y,
                0.0,
                sirina,
                sirina,
            );

            visina_y += sirina;

            for _ in 1..stevilo_ovir {
                // preverimo, ali je zadnja ovira trikotnik
                // ce je, ne dodamo vec nicesar
                if stolp.ovire.last().unwrap().tip == TipOvire::Trikotnik {
                    break;
                }

                let tip = if rand::gen_range(0, 3) == 0 {
                    TipOvire::Trikotnik
                } else {
                    TipOvire::Pravokotnik
                };

                Self::dodaj_v_stolp(
                    &mut stolp,
                    tip,
                    visina_y,
                    0.0,
                    sirina,
                    sirina,
                );

                visina_y += sirina;
            }
        }

        self.poligon.push(stolp);
    }
}


impl Zemljevid for Advanced {
    fn dodaj_oviro(&mut self) {
        self.naredi_stolp();
    }


    fn posodobi(&mut self, dt: f32) {
        // prvih ZAKASNITEV_POSPESKA sekund se hitrost ne spreminja,
        // nato zelo počasi raste v neskončnost
        self.cas += dt;
        if self.cas > ZAKASNITEV_POSPESKA {
            self.hitrost = (self.hitrost + POSPESEK_NA_SEKUNDO * dt).min(NAJVECJA_HITROST);
        }

        for stolp in &mut self.poligon { // gibanje ovir/stolpov
            stolp.x -= self.hitrost;
        }

        // odstrani ovire/stolpe, ko zapustijo ekran
        // (najbolj desno oviro, če jih je več)
        self.poligon.retain(|stolp| {
            let desni_rob = stolp
                .ovire
                .iter()
                .map(|o| o.x_odmik + o.ovira.sirina())
                .fold(0.0, f32::max);

            stolp.x + desni_rob > 0.0
        });

        if self.poligon.is_empty() {
            // dodamo novo oviro/stolp, ce je poligon prazen
            self.dodaj_oviro();
        } else if let Some(zadnji_stolp) = self.poligon.last() {
            // dodamo novo oviro/stolp, ce je desni rob zadnje ovire dovolj dalec
            let desni_rob = zadnji_stolp
                .ovire
                .iter()
                .map(|o| o.x_odmik + o.ovira.sirina())
                .fold(0.0, f32::max)
                + zadnji_stolp.x;

            if desni_rob < screen_width() - zadnji_stolp.razdalja {
                self.dodaj_oviro();
            }
        }
    }


    fn narisi(&self, tla_y: f32) {
        for stolp in &self.poligon {
            for postavitev in &stolp.ovire {
                let x = stolp.x + postavitev.x_odmik;
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
        let mut pristanek = None;
        
        for stolp in &self.poligon {
            for postavitev in &stolp.ovire {
                let y = trenutna_tla - postavitev.y_od_tal;

                match postavitev.ovira.preveri_trk(
                    stolp.x + postavitev.x_odmik,
                    y,
                    igralec,
                ) {
                    IzidTrka::None => {},
                    IzidTrka::Smrt => return IzidTrka::Smrt,
                    IzidTrka::PristaniNaOviri(visina) => {
                        // shranimo in še naprej preverjamo, ali umre
                        pristanek = Some(visina);
                    }
                }
            }
        }
        
        if let Some(visina) = pristanek {
            IzidTrka::PristaniNaOviri(visina)
        } else {
            IzidTrka::None
        }
    }
}
