use macroquad::prelude::*;

pub enum Ovira {

    // Kvadrat deluje kot blok na katerega človeček lahko skoči in se 
    // naprej po njem premika. Če pade iz njega, se igrica nadaljuje. 
    // Če se zaletimo v stranico (ne skočimo nanj), se igrica ustavi.
    Kvadrat {
        stranica: f32,
    },

    //Trikotnik deluje kot spica: če se player zadane vanj, takoj umre.
    Trikotnik {
        visina: f32,
        sirina: f32,
    },
}

impl Ovira {
    pub fn nov_kvadrat(stranica: f32) -> Self {
        Ovira::Kvadrat { stranica }
    }

    pub fn nov_trikotnik(visina: f32, sirina: f32) -> Self {
        Ovira::Trikotnik { visina, sirina }
    }

    pub fn visina(&self) -> f32 {
        match self {
            Ovira::Kvadrat { stranica } => *stranica,
            Ovira::Trikotnik { visina, .. } => *visina,
        }
    }

    pub fn sirina(&self) -> f32 {
        match self {
            Ovira::Kvadrat { stranica } => *stranica,
            Ovira::Trikotnik { sirina, .. } => *sirina,
        }
    }

    pub fn stolpen(&self) -> bool { // true ce lahko na lik damo drug lik
        match self {
            // lahko na kvadrat postavimo še druge ovire, naredimo "stolp"
            Ovira::Kvadrat { .. } => true,
            Ovira::Trikotnik { .. } => false,
        }
    }

    pub fn naredi_stolp(visina: f32) -> Option<Self> {
        if visina == 0. {
            None
        } else {
            Some(Ovira::Kvadrat { 
                stranica: visina
            })
        }
    }

    pub fn draw(&self, color: Color) {
        match self {
            Ovira::Kvadrat { stranica } => {
                let s = *stranica;
                draw_rectangle(
                    // (x,y) je zgornje levo oglišče kvadrata
                    // ta bo postavljen na sredino ekrana
                    screen_width() / 2.0 - s / 2.0,
                    screen_height() / 2.0 - s / 2.0,
                    s,
                    s,
                    color,
                )
            },

            Ovira::Trikotnik { visina, sirina } => {
                let w = *sirina;
                let h = *visina;
                // postavljen na sredino ekrana
                let x = screen_width() / 2.0;
                let y = screen_height() / 2.0;

                // izračun oglišč trikotnika
                let v1 = vec2(x, y - h / 2.0);   // zgornje
                let v2 = vec2(x - w / 2.0, y + h / 2.0); // levo
                let v3 = vec2(x + w / 2.0, y + h / 2.0); // desno

                draw_triangle(v1, v2, v3, color);
            }
        }
    }
}
