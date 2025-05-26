use crate::card::{Figura, Kolor, Karta};
use crate::randgen::{RandGen};
pub struct Talia {
    karty: Vec<Karta>,
    random_gen: RandGen,
}

impl Talia {
    pub fn new() -> Self {
        let mut karty = Vec::new();
        for kolor in [Kolor::Trefl, Kolor::Karo, Kolor::Kier, Kolor::Pik] {
            for n in 2..=10 {
                karty.push(Karta::new(kolor, Figura::Numer(n)));
            }
            karty.push(Karta::new(kolor, Figura::Walet));
            karty.push(Karta::new(kolor, Figura::Dama));
            karty.push(Karta::new(kolor, Figura::Krol));
            karty.push(Karta::new(kolor, Figura::As));
        }
        Self {
            karty, 
            random_gen: RandGen::new(42),
        }
    }

    // pub fn karty(&self) -> Vec<Karta> {
    //     self.karty.clone()
    // }
    pub fn tasuj(&mut self) {
        let len = self.karty.len();
        for i in 0..len {
            let j = self.random_gen.gen_range(i as i64, (len-1) as i64) as usize;
            self.karty.swap(i, j);
        }
    }

    pub fn pobierz_karte(&mut self) -> Option<Karta> {
        self.karty.pop()
    }

    pub fn wyswietl(&self) {
        let mut start = self.karty.len();
        start = if start > 5 {start - 5} else { 0 };
        for karta in &self.karty[start..] {
            println!("{karta:?}");
        }
    }

    pub fn liczba_kart(&self) -> usize {
        self.karty.len()
    }

}
