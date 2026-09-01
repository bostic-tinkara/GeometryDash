use macroquad::prelude::*;
use crate::igra::stanje::IgralnoStanje;

// Funkcija za risanje ozadja z bubble gum temo in dinamiko
pub fn narisi_ozadje(stanje: &IgralnoStanje) {
    // Osnovne barve palete
    let bubble_gum = Color::from_rgba(255, 110, 199, 255);
    let temna_magenta = Color::from_rgba(180, 50, 140, 255);
    let crte_mreze = Color::from_rgba(255, 255, 255, 40); // Rahlo prosojna bela

    clear_background(bubble_gum);

    let sirina = screen_width();
    let visina = screen_height();
    let tla_y = visina - 100.0;

    // 1. Risanje premikajoče se ozadne mreže za občutek globine
    let velikost_mreze = 40.0;
    let odmik_x = (get_time() * 30.0) as f32 % velikost_mreze;

    // Navpične črte
    let mut x = -odmik_x;
    while x < sirina {
        draw_line(x, 0.0, x, tla_y, 1.0, crte_mreze);
        x += velikost_mreze;
    }

    // Vodoravne črte
    let mut y = 0.0;
    while y < tla_y {
        draw_line(0.0, y, sirina, y, 1.0, crte_mreze);
        y += velikost_mreze;
    }

    // 2. Tla z dvobarvnim vzorcem
    draw_rectangle(0.0, tla_y, sirina, 100.0, temna_magenta);
    draw_line(0.0, tla_y, sirina, tla_y, 4.0, WHITE); // Svetleč zgornji rob tal

    // 3. Dodatni dekorativni gradientni sijaj na začetni strani
    if let IgralnoStanje::Meni = stanje {
        let cas = (get_time() * 2.0).sin() as f32 * 5.0;
        draw_circle(sirina / 2.0, visina / 3.0 - 10.0, 180.0 + cas, Color::from_rgba(255, 255, 255, 20));
    }
}
