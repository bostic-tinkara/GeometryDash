use macroquad::prelude::*;
use ::rand::prelude::*;

use crate::ovire::*;

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
                let min_razdalja = 200.0; // min. razdalja med ovirami
                let mut koordinata = 400.0; // tu se zacnejo koordinate ovir
                let list = (0..50)
                .filter_map(|_| {

                    koordinata += rng.random_range(min_razdalja..(min_razdalja + 100.0));

                    if rng.random_bool(1.0 / 3.0) { // se dodatno filtriramo ovire, tretjino jih ignorira
                        return None;
                    }

                    let ovira = if rng.random_bool(0.5) {
                        
                        let dim_kvadrat = rng.random_range(20.0..60.0);
                        Ovira::Pravokotnik { // kvadrat
                            visina: dim_kvadrat, 
                            sirina: dim_kvadrat 
                        }
                    } else {

                        let dim_trikotnik = rng.random_range(30.0..50.0);
                        Ovira::Trikotnik { // enakostranični trikotnik
                            visina: dim_trikotnik * 3.0_f32.sqrt() / 2.0,
                            sirina: dim_trikotnik 
                        }
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