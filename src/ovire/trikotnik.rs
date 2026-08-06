use macroquad::prelude::*;
use crate::igralec::*;
use crate::ovire::IzidTrka;
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
    fn visina(&self) -> f32 {
        self.visina
    }
    
    fn sirina(&self) -> f32 {
        self.sirina
    }

    fn narisi(&self, x: f32, y: f32, color: Color) {
        let h = self.visina;
        let w = self.sirina;

        // izračun oglišč trikotnika // UPORABLJAVA LEVO SPODNJE KRAJIŠČE ZA LAŽJI IZRAČUN POZICIJE
        let v1 = vec2(x + w / 2.0, y - h );   // zgornje
        let v2 = vec2(x , y ); // levo spodaj
        let v3 = vec2(x + w , y ); // desno spodaj
        draw_triangle(v1, v2, v3, color);
    }

    fn preveri_trk(&self, o_x: f32, o_y: f32, p: &Igralec) -> IzidTrka {
        let p_pravokotnik = Rect::new(p.x, p.y, p.stranica, p.stranica); 
        // pravokotnik, ki predstavlja igralca - p

        let h = self.visina;
        let w= self.sirina;

        // Spodnji del trikotnika (širša podlaga)
        let spodnji_hitbox = Rect::new(
            o_x + (w * 0.1),        // Malo ožji od dejanskega dna
            o_y - (h * 0.4),        // Pokriva spodnjih 40% višine
            w * 0.8,
            h * 0.4,
        );
        // Zgornji del trikotnika (ozka špica)
        let zgornji_hitbox = Rect::new(
            o_x + (w * 0.35),       // Močno zamaknjen navznoter, da je ozek
            o_y - h,                // Gre vse do vrha špice
            w * 0.3,                // Širina špice je le 30% celotne širine
            h * 0.6,                // Pokriva zgornjih 60% višine
        );
        if p_pravokotnik.overlaps(&spodnji_hitbox) || p_pravokotnik.overlaps(&zgornji_hitbox) {
            IzidTrka::Smrt
        } else {
            IzidTrka::None
        }
    }
}