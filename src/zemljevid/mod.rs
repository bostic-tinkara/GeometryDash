use macroquad::color::PINK;
use macroquad::prelude::*;
use ::rand::prelude::*;
use ::rand::seq::SliceRandom;

use crate::ovire::*;
use crate::player::*;

pub enum Stopnja {
    Beginner,
}

pub struct Zemljevid {
    pub stopnja: Stopnja,
    pub poligon: Vec<(f32, Ovira)>,
    pub hitrost: f32,
}

impl Zemljevid {
    pub fn new(stopnja: Stopnja) -> Self {
        let poligon = match stopnja {
            Stopnja::Beginner =>
                {
                let mut rng = ::rand::rng();
                let dim = rng.random_range(30.0..60.0);
                let min_razdalja = 150.0;
                let mut koordinata = 400.0;
                let list = (0..20)
                .filter_map(|_| {

                    koordinata += rng.random_range(min_razdalja..min_razdalja + 80.0);
                
                    if !rng.random_bool(0.5) { // se dodatno filtrira ovire
                        return None;
                    }

                    let ovira = if rng.random_bool(0.5) {
                        Ovira::Pravokotnik { visina: dim, sirina: dim }
                    } else {
                        Ovira::Trikotnik { visina: dim, sirina: dim }
                    };
                
                    Some((koordinata, ovira))
                    })
                    .collect();

                list
            
        }};

        Zemljevid {
            stopnja,
            poligon,
            hitrost: 3.,
        }
        
    }

    pub fn posodobi(&mut self) {
        for (x, _) in &mut self.poligon {
            *x -= self.hitrost;
        }
    }

    pub fn narisi(&self, tla_y: f32) {
        for (x, ovira) in &self.poligon {
            if *x > -100.0 && *x < screen_width() + 100.0 { // samo če je ovira blizu ekrana, jo narišemo
                ovira.narisi(*x, tla_y, DARKBLUE);
            }
        }
    }

}