use crate::{deck::Talia, player::Gracz, player::Wynik};



pub struct Oczko {
    talia: Talia,
    gracze: Vec<Gracz>,  
}

impl Oczko {
    pub fn new() -> Self {
        Self { talia: Talia::new(), gracze: Vec::new() }
    }

    pub fn dodaj_gracza(&mut self, gracz: Gracz) {
        self.gracze.push(gracz);
    }

    fn rozegraj_ture(&mut self) {
        for g in &mut self.gracze {
            g.pokaz_reke();
            
            if g.wartosc_reki() > 21 {
                g.set_wynik(Wynik::Przegrana);
            } else if g.wartosc_reki() == 21 {
                g.set_wynik(Wynik::Wygrana);
            }

            if g.wynik() == Wynik::WGrze {
                g.pobierz_karte(&mut self.talia);
            }
        }
    }

    pub fn graj(&mut self) {
        self.talia.tasuj();
        while self.gracze.iter()
                .any(|g| g.wynik() == Wynik::WGrze) {
            self.rozegraj_ture();
        }
    }




}