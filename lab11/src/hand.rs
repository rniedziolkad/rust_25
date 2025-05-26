use crate::card::{Figura, Karta};

pub struct Reka {
    karty: Vec<Karta>,
}

impl Reka {
    pub fn new() -> Self {
        Self { karty: Vec::new() }
    }

    pub fn dodaj_karte(&mut self, karta:Karta) {
        self.karty.push(karta);
    }

    pub fn wartosc(&self) -> u8 {
        let mut suma = 0;
        let mut liczba_asow = 0;
        for karta in &self.karty {
            match karta.figura() {
                Figura::Numer(n) => suma += n,
                Figura::Walet => suma += 2,
                Figura::Dama => suma += 3,
                Figura::Krol => suma += 4,
                Figura::As => {
                    suma += 11;
                    liczba_asow += 1;
                },
            }
        };
        while suma > 21 && liczba_asow > 0{
            suma -= 10;
            liczba_asow -= 1;
        }
        
        suma
    }

    pub fn wyswietl(&self) {
        for karta in &self.karty {
            println!("\t{:?} {:?}", karta.figura(), karta.kolor());
        }
    }

    // pub fn liczba_kart(&self) -> usize {
    //     self.karty.len()
    // }
}