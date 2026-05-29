use crate::ovire::*;
use macroquad::prelude::*;

pub struct Player {
    pub x: f32, // koordinati, kjer se nahaja
    pub y: f32,
    pub stranica: f32,
    pub skok: f32,      // koliko lahko preskoči
    pub y_hitrost: f32, // Nova spremenljivka za hitrost skoka/padanja
    pub skok_moc: f32,  // Kako močan je začetni odriv
    pub rotacija: f32,
}

impl Player {
    pub fn new(stranica: f32, screen_height: f32) -> Self {
        // naredi novega igralca na (x, y) mestu
        Player {
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

    pub fn lahko_preskoci(&self, ovira: &Ovira) -> bool {
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
        let sredina_stranice = s / 2.0;
        draw_poly_lines(
            self.x + s / 2.0, 
            self.y + s / 2.0, 
            4, 
            s * 0.707, // Matematika (s / kvadratni koren iz 2), da se ujema z oglišči kocke
            self.rotacija + 45.0, // Zamik za 45 stopinj, da se ujema s kvadratom
            1.5, 
            WHITE
            
        );
        draw_rectangle_ex(
            // igralčeva oblika je kvadrat
            self.x,
            self.y,
            s,
            s,
            DrawRectangleParams{
                offset: vec2(-sredina_stranice,  -sredina_stranice), // nastavimo offset, da se kvadrat vrti okoli svojega središča
                rotation: self.rotacija.to_radians(), //hočemo radiane
                color,
            },
        );
    }
}
