#[derive(Debug, PartialEq, Clone)]
enum Jednostka {
    Sztuki(f64),
    Litry(f64),
    Kilogramy,
}

#[derive(Debug, PartialEq, Clone)]
enum Przechowywanie {
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(Debug, PartialEq, Clone)]
struct Towar {
    nazwa: String,
    jednostka: Jednostka,
    // waga_jednostkowa: f64,
    przechowywanie: Przechowywanie,
}

impl Towar {
    fn new(nazwa: &str, 
        jednostka: Jednostka, 
        przechowywanie: Przechowywanie
    ) -> Result<Self, String> {

        match jednostka {
            Jednostka::Litry(waga) => {
                if waga <= 0.0 {
                    return Err("Waga dla litrów musi być dodatnia".to_string());
                }
            },
            Jednostka::Sztuki(waga) => {
                if waga <= 0.0 {
                    return Err("Waga dla sztuk musi być dodatnia".to_string());
                }
            }      
            _ => {},
        }

        Ok(Towar { 
            nazwa: nazwa.to_string(),
            jednostka,
            przechowywanie 
        })
    }

    fn waga_jednostkowa(&self) -> f64 {
        match self.jednostka {
            Jednostka::Kilogramy => return 1.0,
            Jednostka::Sztuki(x) | Jednostka::Litry(x) => return x,
        }
    }
}

#[derive(Debug)]
struct ElementZamowienia {
    towar: Towar,
    ilosc: f64,
}

#[derive(Debug)]
struct Zamowienie {
    elementy: Vec<ElementZamowienia>,
}

impl Zamowienie {
    fn new() -> Self {
        Self { elementy: Vec::new() }
    }

    fn dodaj(&mut self, towar: &Towar, ilosc: f64) -> Result<(), String> {
        if ilosc <= 0.0 {
            return Err("Ilość musi być dodatnia".to_string());
        }

        if let Jednostka::Sztuki(_) = towar.jednostka {
            if ilosc.fract() != 0.0 {
                return Err("Sztuki muszą być liczbą całkowitą".to_string());
            }
        }

        let found = self.elementy.iter_mut()
                        .find(|e| e.towar == *towar);

        if let Some(element) = found {
            element.ilosc += ilosc;
        } else {
            self.elementy.push(ElementZamowienia{ towar: towar.clone(), ilosc});
        }

        Ok(())
    }

    fn laczna_waga(&self) -> f64 {
        self.elementy.iter()
            .map(|e| e.towar.waga_jednostkowa()*e.ilosc)
            .sum()
    }

    fn laczna_waga_przechowywanie(&self, przechowywanie: Przechowywanie) -> f64 {
        self.elementy.iter()
            .filter(|e| e.towar.przechowywanie == przechowywanie)
            .map(|e| e.towar.waga_jednostkowa()*e.ilosc)
            .sum()
    }
}

fn main() {
    let t1 = Towar::new(
        "schab", 
        Jednostka::Kilogramy, 
        Przechowywanie::Zamrazarka
    ).unwrap();
    
    println!("{t1:?}");
    
    let t2 = Towar::new(
        "mleko",
        Jednostka::Litry(1.03),
        Przechowywanie::Chlodziarka,
    ).unwrap();
    println!("{t2:?}");
    
    let t2err = Towar::new(
        "mleko",
        Jednostka::Litry(0.0),
        Przechowywanie::Chlodziarka,
    );
    println!("{t2err:?}");

    let t3 = Towar::new(
        "banan",
        Jednostka::Sztuki(0.2), 
        Przechowywanie::Normalne
    ).unwrap();
    println!("{t3:?}");

    let t3err = Towar::new(
        "banan",
        Jednostka::Sztuki(-0.2), 
        Przechowywanie::Normalne
    );
    println!("{t3err:?}");

    println!("{} {} {}", 
            t1.waga_jednostkowa() == 1.0, 
            t2.waga_jednostkowa() == 1.03, 
            t3.waga_jednostkowa() == 0.2);

    let mut zamowienie = Zamowienie::new();
    println!("{:?}", zamowienie.dodaj(&t1, 13.5).is_ok());
    println!("{:?}", zamowienie.dodaj(&t1, 2.5).is_ok());
    println!("{:?}", zamowienie.dodaj(&t2, 5.0).is_ok());
    println!("{:?}", zamowienie.dodaj(&t3, 4.5).is_err());
    println!("{:?}", zamowienie.dodaj(&t3, 4.0).is_ok());
    println!("{:?}", zamowienie.dodaj(&t2, 7.0).is_ok());

    println!("{}", zamowienie.laczna_waga() == 29.16);
    println!("normalne: {}", 
        zamowienie.laczna_waga_przechowywanie(Przechowywanie::Normalne) 
                    == 0.8);
    println!("chlodziarka : {}", 
        zamowienie.laczna_waga_przechowywanie(Przechowywanie::Chlodziarka) 
                    == 12.36);

    println!("{zamowienie:?}");

}
