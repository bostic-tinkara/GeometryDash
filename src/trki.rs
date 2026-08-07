use macroquad::prelude::*;

// uporabljava metodo za zaznavanje trkov s pomocjo
// Separating Axis Theorem: konveksna mnogokotnika
// se ne sekata, ce obstaja os, na kateri se njuni
// projekciji oz. intervala, ki ju dobimo s projekcijo 
// oglisc na os, ne sekata

fn os(x: Vec2, y: Vec2) -> Vec2 {
    // za osi je dovolj vzeti normale na stranice lika
    let stranica = y - x;
    vec2(-stranica.y, stranica.x).normalize()
}

fn projekcija_lika(oglisca: &[Vec2], os: Vec2) -> (f32, f32) {
    let mut min = oglisca[0].dot(os);
    let mut max = min;

    for &tocka in &oglisca[1..] {
        let projekcija = tocka.dot(os);
        min = min.min(projekcija);
        max = max.max(projekcija);
    }
    (min, max) // interval (projekcija lika)
}

fn prekrivanje_intervalov(a: (f32, f32), b: (f32, f32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

pub fn prekrivanje_likov(prvi: &[Vec2], drugi: &[Vec2]) -> bool {
    for lik in [prvi, drugi] {
        for i in 0..lik.len() {
            let tocka1 = lik[i];
            let tocka2 = lik[(i + 1) % lik.len()]; // % vkljuci zadnjo stranico
            let os = os(tocka1, tocka2);

            let projekcija1 = projekcija_lika(prvi, os);
            let projekcija2 = projekcija_lika(drugi, os);

            if !prekrivanje_intervalov(projekcija1, projekcija2) {
                return false; // nasli smo os, na kateri se ne sekata
            }
        }
    }
    true // nismo nasli taksne osi
}
