use macroquad::prelude::*;
use crate::igralec::*;
use crate::ovire::IzidTrka;
use crate::trki::prekrivanje_likov;
use super::Ovira;

// Kvadrat deluje kot blok na katerega človeček lahko skoči in se 
// naprej po njem premika. Če pade iz njega, se igrica nadaljuje. 
// Če se zaletimo v stranico (ne skočimo nanj), se igrica ustavi.
pub struct Pravokotnik {
    pub visina: f32,
    pub sirina: f32,
}

impl Pravokotnik {
    pub fn kvadrat(sirina: f32) -> Self {
        Pravokotnik {
            visina: sirina,
            sirina,
        }
    }
}

impl Ovira for Pravokotnik {
    fn visina(&self) -> f32 {
        self.visina
    }
    
    fn sirina(&self) -> f32 {
        self.sirina
    }

    fn oglisca(&self, x: f32, y: f32) -> Vec<Vec2> {
    vec![
            vec2(x, y - self.visina),
            vec2(x + self.sirina, y - self.visina),
            vec2(x + self.sirina, y),
            vec2(x, y), // zgornje levo oglisce pravokotnika
        ]
    }

    fn narisi(&self, x: f32, y: f32, color: Color) {
        let v = self.oglisca(x, y);

        draw_rectangle(
            v[0].x,
            v[0].y,
            self.sirina,
            self.visina,
            color,
        );
    }

    fn preveri_trk(&self, o_x: f32, o_y: f32, p: &Igralec) -> IzidTrka {
        let igralec = p.oglisca();
        let ovira = self.oglisca(o_x, o_y);

        if !prekrivanje_likov(&igralec, &ovira) {
            return IzidTrka::None;
        }

        // preverimo, ali smo pristali na kvadratu ali se zaleteli v stranico
        let vrh_ovire = o_y - self.visina;
        if p.y_hitrost > 0.0 // igralec pada pol
            && p.y + p.stranica <= vrh_ovire + p.y_hitrost 
            // igralec je dovolj blizu ovire
        { 
            // pristali smo na pravokotniku
            IzidTrka::PristaniNaOviri(vrh_ovire)
        } else { 
            // zaleteli smo se v stranico
            IzidTrka::Smrt
        }
    }
}
