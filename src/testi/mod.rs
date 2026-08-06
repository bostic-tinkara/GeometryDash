use macroquad::prelude::*;
use crate::igralec::*;
use crate::ovire::Ovira;
use crate::ovire::IzidTrka;
use crate::ovire::trikotnik::Trikotnik;
use crate::ovire::pravokotnik::Pravokotnik;

#[test]
fn test_player_new() {
    let igralec = Igralec::new(10., 800.);
    let ovira = Trikotnik { visina: 130., sirina: 20. };

    assert_eq!(igralec.x, 40.);
    assert_eq!(igralec.stranica, 10.);
    assert_eq!(igralec.skok, 150.);
    assert_eq!(Igralec::lahko_preskoci(&igralec, &ovira), true);
}

#[test]
fn test_preveri_trk() {
    let igralec = Igralec::new(10., 800.);
    let ovira = Pravokotnik { visina: 130., sirina: 20. };
    let trk = ovira.preveri_trk(35., 705., &igralec);
    let izid= match trk {
            IzidTrka::Smrt | IzidTrka::None => 0.,
            IzidTrka::PristaniNaOviri(koordinata) => koordinata
        };

    println!("{:?}", trk);
    assert_eq!(izid, 575.);
}
