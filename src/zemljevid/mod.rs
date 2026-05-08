use crate::{ovire::*, player::*};

pub struct Zemljevid {
    stopnja: Stopnja,
    poligon: Vec<(f32, Ovira)>,
}

enum Stopnja {
    Beginner,
}

impl Zemljevid {
    fn new() -> Self {
        Zemljevid {
            stopnja: Stopnja::Beginner,
            poligon: vec![]
        }
        
    }
}


// work in progress
// struct OviraZemlj {
//     pozicija: f32,
//     lik: Ovire,
// }
