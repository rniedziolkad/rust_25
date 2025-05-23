struct RandGen {
    seed: i64
}

impl RandGen {
    fn new(seed: i64) -> Self {
        Self { seed }
    }

    fn gen_range(&mut self, min: i64, max: i64) -> i64 {
        const A: i64 = 1664525;
        const C: i64 = 1013904223;
        const M: i64 = 1<<32;
        // M (modulo) dzięki wrapping_mul i wrapping_add
        self.seed = (self.seed.wrapping_mul(A).wrapping_add(C)) % M;
        min + (self.seed % (max-min+1))
    }
}

struct Urna {
    rng: RandGen,
    symbols: Vec<char>,
}

impl Urna {
    fn new(rng: RandGen) -> Self {
        Self {
            rng,
            symbols: Vec::new(),
        }
    }

    fn doloz(&mut self, symbol: char) {
        self.symbols.push(symbol);
    }

    fn rozmiar(&self) -> usize {
        self.symbols.len()
    }

    fn losuj_bez_us(&mut self) -> Option<char> {
        if self.symbols.is_empty(){
            return None;
        }

        let i = self.rng.gen_range(0, self.symbols.len() as i64 - 1) as usize;
        Some(self.symbols[i])
    }

    fn losuj_z_us(&mut self) -> Option<char> {
        if self.symbols.is_empty(){
            return None;
        }
        
        let i = self.rng.gen_range(0, self.symbols.len() as i64 - 1) as usize;
        Some(self.symbols.remove(i))
    }

}

fn main() {
    let mut urna = Urna::new(RandGen::new(123));

    let a: Option<char> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<char> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz('a');
    urna.doloz('b');
    urna.doloz('c');
    urna.doloz('d');

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<char> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<char> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<char> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}
