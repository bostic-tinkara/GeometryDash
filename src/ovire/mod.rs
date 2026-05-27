use macroquad::prelude::*;
use crate::player::*;

pub enum Ovira {

    // Kvadrat deluje kot blok na katerega človeček lahko skoči in se 
    // naprej po njem premika. Če pade iz njega, se igrica nadaljuje. 
    // Če se zaletimo v stranico (ne skočimo nanj), se igrica ustavi.
    Pravokotnik {
        visina: f32,
        sirina: f32,
    },

    //Trikotnik deluje kot spica: če se player zadane vanj, takoj umre.
    Trikotnik {
        visina: f32,
        sirina: f32,
    },
}

pub enum IzidTrka {
    None,
    Smrt,
    PristaniNaOviri(f32), // višina, na kateri smo pristali
}

impl Ovira {
    pub fn nov_kvadrat(stranica: f32) -> Self {
        Ovira::Pravokotnik { visina: stranica, sirina: stranica }
    }

    pub fn nov_trikotnik(visina: f32, sirina: f32) -> Self {
        Ovira::Trikotnik { visina, sirina }
    }

    pub fn visina(&self) -> f32 {
        match self {
            Ovira::Pravokotnik { visina, .. } => *visina,
            Ovira::Trikotnik { visina, .. } => *visina,
        }
    }

    pub fn sirina(&self) -> f32 {
        match self {
            Ovira::Pravokotnik { sirina, .. } => *sirina,
            Ovira::Trikotnik { sirina, .. } => *sirina,
        }
    }

    pub fn naredi_stolp(visina: f32) -> Self {
        if visina <= 0. {
            Ovira::Pravokotnik { 
                visina: 0.,
                sirina: 0.
            }
        } else {
            Ovira::Pravokotnik { 
                visina: visina,
                sirina: 50.
            }
        }
    }

    pub fn narisi(&self, x: f32, y: f32, color: Color) {
        match self {
            Ovira::Pravokotnik { visina, sirina } => {
                let v = *visina;
                let s= *sirina;
                draw_rectangle(
                    // (x,y) je zgornje levo oglišče pravokotnika
                    // ta bo postavljen na sredino ekrana
                    x,
                    y - v, //minus visina, da stoji na tleh
                    s,
                    v,
                    color,
                )
            },

            Ovira::Trikotnik { visina, sirina } => {
                let w = *sirina;
                let h = *visina;



                // izračun oglišč trikotnika // UPORABLJAVA LEVO SPODNJE KRAJIŠČE ZA LAŽJI IZRAČUN POZICIJE
                let v1 = vec2(x + w / 2.0, y - h );   // zgornje
                let v2 = vec2(x , y ); // levo spodaj
                let v3 = vec2(x + w , y ); // desno spodaj

                draw_triangle(v1, v2, v3, color);
            }
        }
    }

    pub fn preveri_trk(&self, o_x: f32, o_y:f32, p: &Player) -> IzidTrka {
        let p_pravokotnik = Rect::new(p.x, p.y, p.stranica, p.stranica); 
        // pravokotnik, ki predstavlja igralca - p

        match self {
            Ovira::Pravokotnik { visina, sirina} => {
                let v = *visina;
                let s= *sirina;
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
            },

            Ovira::Trikotnik { visina, sirina } => {
                let w = *sirina;
                let h = *visina;

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
    }

}
