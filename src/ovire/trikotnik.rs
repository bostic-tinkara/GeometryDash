use macroquad::prelude::*;
use crate::igralec::*;
use crate::ovire::IzidTrka;
use crate::trki::prekrivanje_likov;
use super::Ovira;

//Trikotnik deluje kot spica: če se player zadane vanj, takoj umre.
pub struct Trikotnik {
    pub visina: f32,
    pub sirina: f32,
}

impl Trikotnik {
    pub fn enakostranicni(sirina: f32) -> Self {
        Trikotnik {
            visina: sirina * 3.0_f32.sqrt() / 2.0,
            sirina,
        }
    }
}

impl Ovira for Trikotnik {
    fn sirina(&self) -> f32 {
        self.sirina
    }

    fn oglisca(&self, x: f32, y: f32) -> Vec<Vec2> {
    vec![ // izracun oglisc trikotnika
            vec2(x + self.sirina / 2.0, y - self.visina),
            vec2(x, y), // levo spodnje oglisce
            vec2(x + self.sirina, y),
        ]
    }

    fn narisi(&self, x: f32, y: f32, color: Color) {
        let v = self.oglisca(x, y);

        draw_triangle(v[0], v[1], v[2], color);
    }

    fn preveri_trk(&self, o_x: f32, o_y: f32, p: &Igralec) -> IzidTrka {
        let igralec = p.oglisca();
        let ovira = self.oglisca(o_x, o_y);

        if prekrivanje_likov(&igralec, &ovira) {
            IzidTrka::Smrt
        } else {
            IzidTrka::None
        }
    }
}
