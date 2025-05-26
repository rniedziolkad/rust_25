use crate::{deck::Talia, hand::Reka};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Wynik {
    Wygrana,
    Przegrana,
    WGrze,
}

pub struct Gracz {
    imie: String,
    reka: Reka,
    wynik: Wynik,
}

impl Gracz {
    pub fn new(imie: &str) -> Self {
        Self {
            imie: imie.to_string(),
            reka: Reka::new(),
            wynik: Wynik::WGrze,
        }
    }

    pub fn pobierz_karte(&mut self, talia: &mut Talia) {
        let picked = talia.pobierz_karte();
        if let Some(card) = picked {
            self.reka.dodaj_karte(card);
        }
    }

    pub fn pokaz_reke(&self) {
        println!("Ręka gracza '{}':", self.imie);
        self.reka.wyswietl();
        println!("\tWartość: {}", self.reka.wartosc());
    }

    pub fn wartosc_reki(&self) -> u8 {
        self.reka.wartosc()
    }

    pub fn wynik(&self) -> Wynik {
        self.wynik
    }

    pub fn set_wynik(&mut self, wynik: Wynik) {
        self.wynik = wynik;
    }
}