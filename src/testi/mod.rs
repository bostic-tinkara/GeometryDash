#[cfg(test)]
mod tests {
    use crate::{ovire::Ovira, player::Player};

    #[test]
    fn test_kvadrat_stolpen() {
        let k = Ovira::nov_kvadrat(5.);

        assert_eq!(k.stolpen(), true);
    }

    #[test]
    fn test_trikotnik_stolpen() {
        let t = Ovira::nov_trikotnik(4., 6.);

        assert_eq!(t.stolpen(), false);
    }

    #[test]
    fn test_player_new() {
        let igralec = Player::new(10.);

        assert_eq!(igralec.x, 0.);
        assert_eq!(igralec.y, 0.);
        assert_eq!(igralec.stranica, 10.);
        assert_eq!(igralec.skok, 6.5);
    }

     #[test]
    fn test_lahko_preskoci_1() {
        let igralec = Player::new(10.);
        let k1 = Ovira::Kvadrat { stranica: 3. };

        assert_eq!(igralec.lahko_preskoci(k1), true);
    }

    #[test]
    fn test_lahko_preskoci_2() {
        let igralec = Player::new(10.);

        let k2 = Ovira::Kvadrat { stranica: 8. };

        assert_eq!(igralec.lahko_preskoci(k2), false);
    }
}
