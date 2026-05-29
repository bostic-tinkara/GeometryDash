use crate::ovire::*;
use crate::player::*;
use macroquad::prelude::*;

#[test]
fn test_player_new() {
    let igralec = Player::new(10., 800.);
    let ovira = Ovira::Trikotnik { visina: 130., sirina: 20. };

    assert_eq!(igralec.x, 40.);
    assert_eq!(igralec.stranica, 10.);
    assert_eq!(igralec.skok, 150.);
    assert_eq!(Player::lahko_preskoci(&igralec, &ovira), true);
}

#[test]
fn test_preveri_trk() {
    let igralec = Player::new(10., 800.);
    let ovira = Ovira::Pravokotnik { visina: 130., sirina: 20. };
    let trk = Ovira::preveri_trk(&ovira, 35., 705., &igralec);
    let izid= match trk {
            IzidTrka::Smrt | IzidTrka::None => 0.,
            IzidTrka::PristaniNaOviri(koordinata) => koordinata
        };

    println!("{:?}", trk);
    assert_eq!(izid, 575.);
}
