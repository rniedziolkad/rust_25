use lab11::game::Oczko;
use lab11::player::Gracz;

fn main() {
    let mut oczko = Oczko::new();
    let g1 = Gracz::new("Adam");
    let g2 = Gracz::new("Ewa");
    oczko.dodaj_gracza(g1);
    oczko.dodaj_gracza(g2);

    oczko.graj();


}
