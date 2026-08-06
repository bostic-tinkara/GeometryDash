use crate::ovire::*;
use macroquad::{color, prelude::*};

pub struct Igralec {
    pub x: f32, // koordinati, kjer se nahaja
    pub y: f32,
    pub stranica: f32,
    pub skok: f32,      // koliko lahko preskoči
    pub y_hitrost: f32, // Nova spremenljivka za hitrost skoka/padanja
    pub skok_moc: f32,  // Kako močan je začetni odriv
    pub rotacija: f32,
}

impl Igralec {
    pub fn new(stranica: f32, screen_height: f32) -> Self {
        // naredi novega igralca na (x, y) mestu
        Igralec {
            x: 40.,
            y: screen_height - 100. - stranica,
            stranica,
            skok: 150., // lahko preskoči oviro do višine 150
            y_hitrost: 0.,
            skok_moc: -9., //negativno, ker gremo navzgor
            rotacija: 0.,
        }
    }

    pub fn posodobi(&mut self, gravitacija: f32, tla_y: f32) {
        self.y_hitrost += gravitacija;
        self.y += self.y_hitrost;

        if self.y > tla_y - self.stranica {
            self.y = tla_y - self.stranica;
            self.y_hitrost = 0.;
        }
    }

    pub fn lahko_preskoci<T: Ovira>(&self, ovira: &T) -> bool {
        let visina = ovira.visina();
        visina <= self.skok
    }

    pub fn skoci(&mut self) {
        if self.y_hitrost == 0. {
            // lahko skočimo samo, če smo na tleh
            self.y_hitrost = self.skok_moc;
        }
    }

    pub fn narisi(&self, color: Color) {
        let s = self.stranica;
        let pol_stranice = s / 2.0;
        let rotacija = self.rotacija.to_radians();

        // igralec je kvadrat
        draw_rectangle_ex(
            self.x + pol_stranice,
            self.y + pol_stranice,
            s,
            s,
            DrawRectangleParams {
                // nastavimo offset, da se kvadrat vrti okoli svojega središča
                offset: vec2(0.5, 0.5), 
                rotation: rotacija,
                color: color,
            },
        );

        draw_poly_lines( // meja okoli kvadrata
            self.x + pol_stranice,
            self.y + pol_stranice,
            4, 
            s * 0.7071, // Matematika (s / kvadratni koren iz 2), da se ujema z oglišči kocke
            self.rotacija + 45.0, // Zamik za 45 stopinj, da se ujema s kvadratom
            1.5,
            WHITE
        );
    }

}
