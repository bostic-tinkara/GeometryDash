use crate::ovire::*;
use macroquad::{color, prelude::*};

pub struct Igralec {
    pub x: f32, // koordinati, kjer se nahaja
    pub y: f32, // (x, y) je zgornje levo oglisce
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

    pub fn oglisca(&self) -> Vec<Vec2> {
        let sredisce = vec2(
            self.x + self.stranica / 2.0,
            self.y + self.stranica / 2.0
        );
        let kot = self.rotacija.to_radians();
        let pol = self.stranica / 2.0;

        vec![
            sredisce + vec2(-pol, -pol).rotate(Vec2::from_angle(kot)),
            sredisce + vec2( pol, -pol).rotate(Vec2::from_angle(kot)),
            sredisce + vec2( pol,  pol).rotate(Vec2::from_angle(kot)),
            sredisce + vec2(-pol,  pol).rotate(Vec2::from_angle(kot)),
        ]
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
        let pol = s / 2.0;
        let rotacija = self.rotacija.to_radians();

        // igralec je kvadrat
        draw_rectangle_ex(
            self.x + pol,
            self.y + pol,
            s,
            s,
            DrawRectangleParams {
                // nastavimo offset, da se kvadrat vrti okoli svojega središča
                offset: vec2(0.5, 0.5),
                rotation: rotacija,
                color,
            },
        );

        // meja okoli kvadrata
        let v = self.oglisca();
        for i in 0..4 {
            let a = v[i];
            let b = v[(i + 1) % 4];

            draw_line(
                a.x, a.y, b.x, b.y, 
                1.5, 
                WHITE
            );
        }
    }
}
