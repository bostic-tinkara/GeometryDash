use crate::ovire::*;
use macroquad::prelude::*;

pub struct Player {
    pub x: f32, // koordinati, kjer se nahaja
    pub y: f32,
    pub stranica: f32,
    pub skok: f32,
}

impl Player {
    pub fn new(stranica: f32) -> Self {
        // naredi novega igralca na x, y ( (0,0) mestu)
        Player {
            x: 0.,
            y: 0.,
            stranica,
            skok: 6.5,
        }
    }

    pub fn lahko_preskoci(&self, ovira: Ovira) -> bool {
        let visina = ovira.visina();
        visina <= self.skok
    }

    pub fn draw(&self, color: Color) {
        let s = self.stranica;
        draw_rectangle( // igralčeva oblika je kvadrat
            self.x,
            self.y,
            s,
            s,
            color,
        )
    }

    // pub fn premik(&mut self, dy: f32) {
    //     self.y = self.y + dy;
    // }
}
