use lab12::rng::RandGen;
use lab12::urna::Urna;

fn main() {
    let mut urna = Urna::<String>::new(RandGen::new(123));

    let a: Option<String> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<String> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz("ala".to_string());
    urna.doloz("ma".to_string());
    urna.doloz("kota".to_string());
    urna.doloz("psa".to_string());

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<String> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<String> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<String> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}