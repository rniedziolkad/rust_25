use lab12::rng::RandGen;
use lab12::urna::Urna;

fn main() {
    let mut urna = Urna::<i32>::new(RandGen::new(123));

    let a: Option<i32> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(1);
    urna.doloz(2);
    urna.doloz(3);
    urna.doloz(4);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<i32> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<i32> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}