use crate::ovire::{self, *};

pub enum Izgled {
    Kvadrat,
}

pub struct Player {
    pub x: f64, // koordinati, kjer se nahaja
    pub y: f64,
    pub visina: u64,
    pub sirina: u64,
    pub izgled: Izgled,
    pub skok: f64,
}

impl Player {
    pub fn new() -> Self {
        // naredi novega igralca na x, y ( (0,0) mestu)
        Player {
            x: 0.,
            y: 0.,
            visina: 1,
            sirina: 1,
            izgled: Izgled::Kvadrat,
            skok: 6.5,
        }
    }

    pub fn lahko_preskoci<T: Ovira>(&self, ovira: T) -> bool {
        let visina = ovira.visina() as f64;
        visina <= self.skok
    }

    // pub fn premik(&mut self, dy: f64) {
    //     self.y = self.y + dy;
    // }
}
