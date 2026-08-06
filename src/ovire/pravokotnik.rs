use macroquad::prelude::*;
use crate::igralec::*;
use crate::ovire::IzidTrka;
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

    fn narisi(&self, x: f32, y: f32, color: Color) {
        let v = self.visina;
        let s = self.sirina;
        draw_rectangle(
            // (x,y) je zgornje levo oglišče pravokotnika
            // ta bo postavljen na sredino ekrana
            x,
            y - v, //minus visina, da stoji na tleh
            s,
            v,
            color,
        )
    }

    fn preveri_trk(&self, o_x: f32, o_y: f32, p: &Igralec) -> IzidTrka {
        let p_pravokotnik = Rect::new(p.x, p.y, p.stranica, p.stranica); 
        // pravokotnik, ki predstavlja igralca - p

        let v = self.visina;
        let s= self.sirina;
        let o_pravokotnik = Rect::new(o_x, o_y - v, s, v);
        if p_pravokotnik.overlaps(&o_pravokotnik) {
            // preverimo, ali smo pristali na kvadratu ali se zaleteli v stranico
            if p.y + p.stranica <= o_y - 5.0 { // pristali smo na kvadratu, 5.0 višje, da program pravočasno zazna
                IzidTrka::PristaniNaOviri(o_y - v)
            } else { // zaleteli smo se v stranico
                IzidTrka::Smrt
            }
        } else {
            IzidTrka::None
        }
    }
}
