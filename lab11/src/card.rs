#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kolor {
    Trefl,
    Karo,
    Kier,
    Pik,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Figura {
    Numer(u8),
    Walet,
    Dama,
    Krol,
    As
}

#[derive(Debug, Clone, PartialEq)]
pub struct Karta {
    kolor: Kolor,
    figura: Figura,
}

impl Karta {
    pub fn new(kolor: Kolor, figura: Figura) -> Self {
        Self{ kolor, figura }
    }

    pub fn kolor(&self) -> Kolor {
        self.kolor
    }

    pub fn figura(&self) -> Figura {
        self.figura
    }

}


