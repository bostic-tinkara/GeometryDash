use macroquad::prelude::*;
use macroquad::audio::{play_sound, stop_sound, PlaySoundParams, Sound};

use crate::igra::gumbi;
use crate::igralec::Igralec;
use crate::zemljevid::{Stopnja, Zemljevid, naredi_zemljevid};
use crate::{IgralnoStanje, ovire::IzidTrka};

const GRAVITACIJA: f32 = 0.35;
const VISINA_TAL: f32 = 150.0;

/// Katera glasba v zanki se trenutno predvaja.
#[derive(PartialEq, Clone, Copy)]
enum Predvajano {
    Nobena,
    Meni,
    Igra,
}

pub struct Igra {
    pub igralec: Igralec,
    pub stopnja: Stopnja,
    pub zemljevid: Box<dyn Zemljevid>,
    pub score_accumulator: f32,
    pub best_score: u32,
    pub dead: bool,
    pub cas_smrti: f64,
    pub glasba: Option<Sound>,
    pub glasba_meni: Option<Sound>,
    pub smrt_jingle: Option<Sound>,
    predvajano: Predvajano,
    smrt_odigrana: bool,
}


impl Igra {
    pub fn new(stopnja: Stopnja) -> Self {
        Self {
            igralec: Igralec::new(40.0, screen_height()),
            stopnja,
            zemljevid: naredi_zemljevid(&stopnja),
            score_accumulator: 0.0,
            best_score: 0,
            dead: false,
            cas_smrti: 0.0,
            glasba: None,
            glasba_meni: None,
            smrt_jingle: None,
            predvajano: Predvajano::Nobena,
            smrt_odigrana: false,
        }
    }

    /// Uskladi predvajanje glasbe s stanjem igre:
    /// - v meniju in v pavzi (resume) se vrti glasba menija,
    /// - med igranjem glasba ozadja,
    /// - ob smrti se enkrat predvaja kratek jingle, glasba ozadja pa utihne.
    pub fn posodobi_glasbo(&mut self, stanje: &IgralnoStanje) {
        // Enkratni jingle ob smrti.
        if self.dead {
            if !self.smrt_odigrana {
                if let Some(jingle) = &self.smrt_jingle {
                    play_sound(jingle, PlaySoundParams { looped: false, volume: 0.6 });
                }
                self.smrt_odigrana = true;
            }
        } else {
            self.smrt_odigrana = false;
        }

        // Katera glasba v zanki naj se predvaja.
        let zeljena = match stanje {
            IgralnoStanje::Meni | IgralnoStanje::Pavza => Predvajano::Meni,
            IgralnoStanje::Igra if !self.dead => Predvajano::Igra,
            _ => Predvajano::Nobena,
        };

        if zeljena == self.predvajano {
            return;
        }

        // Ustavimo prejšnjo skladbo.
        match self.predvajano {
            Predvajano::Meni => {
                if let Some(s) = &self.glasba_meni {
                    stop_sound(s);
                }
            }
            Predvajano::Igra => {
                if let Some(s) = &self.glasba {
                    stop_sound(s);
                }
            }
            Predvajano::Nobena => {}
        }

        // Zaženemo novo skladbo.
        match zeljena {
            Predvajano::Meni => {
                if let Some(s) = &self.glasba_meni {
                    play_sound(s, PlaySoundParams { looped: true, volume: 0.5 });
                }
            }
            Predvajano::Igra => {
                if let Some(s) = &self.glasba {
                    play_sound(s, PlaySoundParams { looped: true, volume: 0.5 });
                }
            }
            Predvajano::Nobena => {}
        }

        self.predvajano = zeljena;
    }

    pub fn posodobi(&mut self, stanje: &mut IgralnoStanje) {
        // ob trku
        if self.dead {
            self.posodobi_smrt();
            return;
        }

        let dt = get_frame_time();

        // set up za igro
        self.zemljevid.posodobi(dt);

        self.score_accumulator += 10.0 * dt;

        let osnovna_tla = screen_height() - VISINA_TAL;
        let mut trenutna_tla = osnovna_tla;
        let mut na_oviri = false;

        let miska_nad_meni_gumbom = self.preveri_pavzo(stanje);
        
        self.preveri_skok(miska_nad_meni_gumbom);
        
        self.posodobi_igralca(&mut trenutna_tla, &mut na_oviri, osnovna_tla, dt);
    }


    pub fn narisi(&self) {
        //tla, kjer je igralec, če ni na nobeni oviri 
        // (lahko je tudi na oviri, ne samo na tleh)
        let osnovna_tla = screen_height() - VISINA_TAL;
        let current_score = self.score_accumulator as u32;

        self.zemljevid.narisi(osnovna_tla);

        // Risanje ob smrti
        if self.dead {
            // ko igralec umre, utripa
            let sinus = (get_time() * 10.0).sin();
            if sinus > 0.0 {
                self.igralec.narisi(BLUE);
            }

        } else {
            // RISANJE IGRE
            self.igralec.narisi(BLUE);
        }

        // RISANJE "RESUME/PAUSE" GUMBA
        let (x, y, velikost) = gumbi::meni_gumb();
        let barva_meni_gumba = if gumbi::miska_nad_meni_gumbom() { DARKBLUE } else { BLUE };

        // kvadratna podlaga z belim robom
        draw_rectangle(x, y, velikost, velikost, barva_meni_gumba);
        draw_rectangle_lines(x, y, velikost, velikost, 2.5, WHITE);

        // narisemo dve beli navpični črtici (Pauza ikona: ❚❚)
        let pas_sirina = 6.0;
        let pas_visina = 20.0;
        let odmerek_y = y + 10.0;

        draw_rectangle(x + 11.0, odmerek_y, pas_sirina, pas_visina, WHITE);
        draw_rectangle(x + 23.0, odmerek_y, pas_sirina, pas_visina, WHITE);

        draw_text(
            &format!("Score: {}", current_score),
            10.0,
            30.0,
            30.0,
            WHITE,
        );
    }


    fn posodobi_smrt(&mut self) {
        if get_time() - self.cas_smrti > 2.0 {
            // igra se ponovno začne
            self.ponovno_zazeni();
        }
    }


    fn preveri_pavzo(&mut self, stanje: &mut IgralnoStanje) -> bool {
        let miska_nad_meni_gumbom = gumbi::miska_nad_meni_gumbom();

        // Če kliknemo gumb za Meni, preklopimo stanje
        if miska_nad_meni_gumbom && is_mouse_button_pressed(MouseButton::Left) {
            let current_score = self.score_accumulator as u32;
            if current_score > self.best_score {
                self.best_score = current_score;
            }

            *stanje = IgralnoStanje::Pavza;
        }

        miska_nad_meni_gumbom
    }


    fn preveri_skok(&mut self, miska_nad_meni_gumbom: bool) {
        // (Preprečimo skok ob kliku na gumb) ---
        let klik_za_skok = 
            is_mouse_button_pressed(MouseButton::Left) && !miska_nad_meni_gumbom;
        
        if (is_key_pressed(KeyCode::Space) || klik_za_skok)
            && self.igralec.skoki < self.stopnja.najvec_skokov()
        {
            self.igralec.skoci();
            self.igralec.skoki += 1;
        }
    }

    fn posodobi_igralca(
        &mut self,
        trenutna_tla: &mut f32,
        na_oviri: &mut bool,
        osnovna_tla: f32,
        dt: f32
    ) {
        // ponastavimo
        *na_oviri = false;
        *trenutna_tla = osnovna_tla;

        self.igralec.posodobi(GRAVITACIJA, *trenutna_tla);

        match self.zemljevid.preveri_trk(*trenutna_tla, &self.igralec) {
            IzidTrka::None => {}
            IzidTrka::Smrt => {
                self.dead = true;
                self.cas_smrti = get_time();

                let current_score = self.score_accumulator as u32;
                if current_score > self.best_score {
                    self.best_score = current_score;
                }
            }
            IzidTrka::PristaniNaOviri(visina) => {
                *na_oviri = true;
                *trenutna_tla = visina;
                // igralec pristane zgoraj
                self.igralec.y = *trenutna_tla - self.igralec.stranica;
                self.igralec.y_hitrost = 0.0;
                self.igralec.rotacija = 0.0;
                self.igralec.skoki = 0; // ponastavimo število možnih skokov
            }
        }

        // Rotacija
        let na_tleh = self.igralec.y >= *trenutna_tla - self.igralec.stranica - 0.5;

        if na_tleh && self.igralec.y_hitrost >= 0.0 {
            self.igralec.je_skocil = false;
            self.igralec.rotacija = 0.0;
            self.igralec.skoki = 0;
        } else if self.igralec.je_skocil {
            // ob skoku se ves čas vrti
            self.igralec.rotacija += 400.0 * dt;
        } else {
            // poravnan pade dol z ovir
            self.igralec.rotacija = 0.0;
        }

    }

    pub fn ponovno_zazeni(&mut self) {
        self.score_accumulator = 0.0;
        self.zemljevid = naredi_zemljevid(&self.stopnja);

        let osnovna_tla = screen_height() - VISINA_TAL;
        self.igralec.y = osnovna_tla - self.igralec.stranica;
        self.igralec.y_hitrost = 0.0;
        self.igralec.rotacija = 0.0;

        self.dead = false;
    }
}
