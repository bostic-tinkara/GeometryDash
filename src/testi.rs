use macroquad::prelude::*;
use crate::igralec::*;
use crate::zemljevid::Stopnja;
use crate::trki::*;


#[test]
fn beginner_ima_en_skok() {
    assert_eq!(Stopnja::Beginner.najvec_skokov(), 1);
}

#[test]
fn advanced_ima_tri_skoke() {
    assert_eq!(Stopnja::Advanced.najvec_skokov(), 3);
}

#[test]
fn skok_nastavi_hitrost() {
    let mut igralec = Igralec::new(40.0, 800.0);

    igralec.skoci();

    assert_eq!(igralec.y_hitrost, igralec.skok_moc);
}

#[test]
fn intervala_se_prekrivata() {
    assert!(prekrivanje_intervalov((0.0, 5.0), (3.0, 8.0)));
}

#[test]
fn intervala_se_ne_prekrivata() {
    assert!(!prekrivanje_intervalov((0.0, 2.0), (3.0, 5.0)));
}

#[test]
fn intervala_se_dotikata() {
    assert!(prekrivanje_intervalov((0.0, 3.0), (3.0, 5.0)));
}

#[test]
fn prekrivanje_likov_deluje() {
    let prvi = vec![
        vec2(0.0, 0.0),
        vec2(10.0, 0.0),
        vec2(10.0, 10.0),
        vec2(0.0, 10.0),
    ];

    let drugi = vec![
        vec2(5.0, 5.0),
        vec2(15.0, 5.0),
        vec2(15.0, 15.0),
        vec2(5.0, 15.0),
    ];

    assert!(prekrivanje_likov(&prvi, &drugi));
}

#[test]
fn igralec_se_pravilno_ustvari() {
    let igralec = Igralec::new(40.0, 800.0);

    assert_eq!(igralec.stranica, 40.0);
    assert_eq!(igralec.y_hitrost, 0.0);
    assert_eq!(igralec.skoki, 0);
    assert_eq!(igralec.rotacija, 0.0);
}

#[test]
fn skok_spremeni_hitrost() {
    let mut igralec = Igralec::new(40.0, 800.0);

    igralec.skoci();

    assert_eq!(igralec.y_hitrost, igralec.skok_moc);
}

#[test]
fn igralec_lahko_skoci_v_zraku() {
    let mut igralec = Igralec::new(40.0, 800.0);

    igralec.y_hitrost = 2.0;
    igralec.skoci();

    assert_eq!(igralec.y_hitrost, igralec.skok_moc);
}

#[test]
fn skoki_se_povecajo() {
    let mut igralec = Igralec::new(40.0, 800.0);

    assert_eq!(igralec.skoki, 0);

    igralec.skoci();
    igralec.skoki += 1;
    assert_eq!(igralec.skoki, 1);

    igralec.skoci();
    igralec.skoki += 1;
    assert_eq!(igralec.skoki, 2);

    igralec.skoci();
    igralec.skoki += 1;
    assert_eq!(igralec.skoki, 3);
}
