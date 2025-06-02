use crate::rng::RandGen;

pub struct Urna<T> {
    rng: RandGen,
    symbols: Vec<T>,
}

impl<T: Clone> Urna<T> {
    pub fn new(rng: RandGen) -> Self {
        Self {
            rng,
            symbols: Vec::new(),
        }
    }

    pub fn doloz(&mut self, symbol: T) {
        self.symbols.push(symbol);
    }

    pub fn rozmiar(&self) -> usize {
        self.symbols.len()
    }

    pub fn losuj_bez_us(&mut self) -> Option<T> {
        if self.symbols.is_empty(){
            return None;
        }

        let i = self.rng.gen_range(0, self.symbols.len() as i64 - 1) as usize;
        Some(self.symbols[i].clone())
    }

    pub fn losuj_z_us(&mut self) -> Option<T> {
        if self.symbols.is_empty(){
            return None;
        }
        
        let i = self.rng.gen_range(0, self.symbols.len() as i64 - 1) as usize;
        Some(self.symbols.remove(i))
    }

}
