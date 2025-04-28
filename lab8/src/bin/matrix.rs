#[derive(Debug)]
struct Macierz {
    wysokosc: usize,
    szerokosc: usize,
    dane: Vec<Vec<f64>>,
}

impl Macierz {
    fn new(wysokosc: usize, szerokosc: usize, wypelniacz: f64) -> Macierz {
        Self{
            wysokosc,
            szerokosc,
            dane: vec![vec![wypelniacz; szerokosc]; wysokosc],
        }
    }

    fn zerowa(wysokosc: usize, szerokosc: usize) -> Macierz {
        Self::new(wysokosc, szerokosc, 0.0)
    }

    fn jednostkowa(wysokosc: usize) -> Macierz {
        let mut res = Self::zerowa(wysokosc, wysokosc);
        for i in 0..res.wysokosc {
            res.dane[i][i] = 1.0;
        }
        res
    }

    fn element(&self, indeks_wiersza: usize, indeks_kolumny: usize) -> f64 {
        self.dane[indeks_wiersza][indeks_kolumny]
    }

    fn zmien_element(&mut self, indeks_wiersza: usize, indeks_kolumny: usize, nowa_wartosc: f64) {
        self.dane[indeks_wiersza][indeks_kolumny] = nowa_wartosc;
    }

    fn suma(macierz1: &Self, macierz2: &Self) -> Option<Macierz> {
        if macierz1.wysokosc != macierz2.wysokosc || macierz1.szerokosc != macierz2.szerokosc {
            return None;
        }
        let mut res = Self::zerowa(macierz1.wysokosc, macierz2.szerokosc);
        for i in 0..macierz1.wysokosc {
            for j in 0..macierz1.szerokosc {
                res.dane[i][j] = macierz1.dane[i][j] + macierz2.dane[i][j];
            }
        }
        Some(res)
    }

    fn wyswietl(&self) {
        for i in 0..self.wysokosc {
            for j in 0..self.szerokosc {
                print!("{} ", self.dane[i][j]);
            }
            print!("\n");
        }
    }
}

fn main() {
    let filled1s = Macierz::new(5, 5, 1.0);
    println!("{} {}", filled1s.wysokosc == 3, filled1s.szerokosc == 4);
    let zeros = Macierz::zerowa(4, 4);
    println!("{} {}", zeros.wysokosc == 4, zeros.szerokosc == 4);
    let mut identity = Macierz::jednostkowa(5);
    println!("{} {}", identity.wysokosc == 5, identity.szerokosc == 5);

    filled1s.wyswietl();
    println!();
    zeros.wyswietl();
    println!();
    identity.wyswietl();
    println!();

    println!("{} {}", identity.element(1, 1) == 1.0, identity.element(4, 4) == 1.0);
    println!("{} {}", identity.element(0, 1) == 0.0, identity.element(4, 0) == 0.0);
    identity.zmien_element(2, 2, 5.0);
    println!("{}", identity.element(2, 2) == 5.0);
    identity.wyswietl();
    println!();

    let suma_ok = Macierz::suma(&filled1s, &identity).unwrap();
    println!("{} {}", suma_ok.element(2, 2) == 6.0, suma_ok.element(0, 0) == 2.0);
    suma_ok.wyswietl();
    println!();

    let suma_bad = Macierz::suma(&zeros, &identity);
    println!("{}", suma_bad.is_none());
    

}
