fn takes_ownership(s: String) {
    println!("s w funkcji: '{s}'");
}

fn makes_copy(x: i32) {
    println!("x w funkcji: {x}");
}

fn gives_ownership() -> String {
    let s = String::from("z funkcji");
    s
}

fn takes_and_gives(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" ze zmianą");
}
/*
// wisząca referencja
fn dangle() -> &String {
    let s = String::from("dangled");
    &s      // referencja do usuniętej pamięci
}           // s jest usunięte z pamięci po wyjściu poza zakres
*/

fn main(){
    // print_ascii();
    
    // stos i sterta (kopiec)
    // wszystkie dane na stosie muszą mieć znany, stały rozmiar
    // na stercie przechowywane sa dynamicznie tworzone struktury danych
    
    // Zasady własności:
    // + każda wartość w Rust ma właściciela
    // + w danym momencie może być tylko jeden właściciel
    // + gdy właściciel wyjdzie poza zakres, wartość zostanie usunięta
    #[allow(unused)]
    {                       // s jeszcze nie zadeklarowane
        let s = String::from("hello");  // s obowiązuje od tego momentu

        // zrób coś z s
    }                       // zakres się zakończył, s juz nie obowiązuje; 
                            // wartość usunięta (wywołana funkcja drop)
                            // zgodnie ze wzorcem RAII (Resource Acquisition Is Initialization)

    // kopiowanie vs przenoszenie
    let x = 5;
    let y = x;                      //wykonała się kopia, bo typy proste przechowywane są na stosie i można trywialnie skopiować
    println!("x={x} y={y}");
    
    let s1 = String::from("hello");  
    let s2 = s1;                    // wartość s1 została przeniesiona do s2
    // println!("s1 = '{s1}'");     nie da się, wartość przeniesiona
    println!("s2 = '{s2}'");
    // dzięki przeniesieniu istnieje tylko 1 właściciel i nie dojdzie do błedu podwójnego zwolnienia pamięci
    
    let s3 = s2.clone();            // jawna, głęboka kopia
    println!("s3 = '{s3}'");
    
    // Typy przechowywane na stosie z automatycznym kopiowaniem (posiadają trait (cechę) Copy):
    // + typy proste
    // + tablice o ustalonym rozmiarze
    // + krotki, jeżeli wszystkie elementy są kopiowalne
    
    // tablica -- niemodyfikowalny rozmiar, jeden typ danych
    let arr = ["poniedziałek", "wtorek", "środa", "czwartek", "piątek"];
    println!("arr = {:?}", arr);
    // [typ; rozmiar] = [wartość; rozmiar]
    let arr: [i8; 3]= [10; 3];
    // wyjście poza zakres tablicy -- panic!
    for i in 0..=2/*3*/ {
        println!("arr[{i}] = {}", arr[i]);
    }
    
    // krotka -- grupuje wartości różnych typów w jeden obiekt złożony
    let tup: (u8, f64, i32) = (1, 3.1415, 12345);
    println!("tup = {:?}", tup);
    // możemy użyć dopasowywania wzorców, aby zdestrukturyzować wartość krotki
    #[allow(unused)]
    let (a, b, c) = tup;
    println!("b = {b}");
    println!("tup.2 = {}", tup.2);
    
    
    // własność i funkcje -------------------------------------------------------------------
    let s = String::from("ownership");
    takes_ownership(s);             // wartość s jest przeniesiona do funkcji
    //println!("s = '{s}'");           s już nie obowiązuje
    
    let x = 3;
    makes_copy(x);                  // x jest kopiowane, bo i32 implementuje trait(cechę) Copy
    println!("x = {x}");            // więc x wciąż obowiązuje
    
    let s1 = gives_ownership();     // funkcja przenosi swoją wartość zwracaną do s1
    println!("s1 = '{s1}'");
    let s2 = takes_and_gives(s1);   // s1 przeniesione do funkcji, która przenosi swoją wartość zwracaną do s2
    // println!("s1 = '{s1}'");        wartość z s1 jest już przeniesiona, s1 nie obowiązuje
    println!("s2 = '{s2}'");
    
    // funkcja nam "zabierze" wartość s2, możemy zwrócić 2 wartości
    let (s2, length) = calculate_length(s2);    
    println!("Długość '{s2}' = {length}");
    
    
    // referencje i pożyczanie ------------------------------------------------------
    // zamiast oddawać i zwracać własność przy każdym wywołaniu funkcji
    // możemy "pożyczyć" za pomocą referencji
    let length = calculate_length_ref(&s2);
    println!("Ref. Długość '{s2}' = {length}");
    
    let mut s = String::from("Tekst");
    change(&mut s);
    println!("s = '{s}'");
    
    #[allow(unused)]
    let r1 = &mut s;
    let r2 = &mut s;
    // println!("r1 = '{r1}'");     //możemy mieć tylko 1 referencję mutowalną
    println!("r2 = '{r2}'");
    
    let r1 = &s;                        
    let r2 = &s;
    println!("r1 = {r1}, r2 = {r2}");           // możemy mieć wiele referencji niemutowalnych
    let r3 = &mut s;
    // println!("r3 = {r3}, r2 = {r2}, r1 = {r1}"); // ale nie możemy ich łączyć z mutowalną referencją
    
    // r1 i r2 nie są dalej używane, ich zakres kończy się po poprzednim println!, gdzie są użyte po raz ostatni
    println!("r3 = {r3}");      // więc nie ma problemu  
    
    /* Zasady referencji:
    *  - w dowolnym momencie możemy mieć dokładnie jedną mutowalną referencję 
    *    albo wiele wiele referencji niemutowalnych
    *  - referencje muszą zawsze być prawidłowe (valid), tzn. wskazywać na pamięć, która nie jest zwolniona
    */
    
    // z2
    let n = 12;
    println!("{n}, liczba iteracji: {}", collatz(n));
    
    // z3
    let n = 155;
    println!("{n}, Armstronga ? {}", armstrong(n));
    
    // z4
    let n = 496;
    println!("{n}, doskonała ? {}", doskonala(n));
    
    // z5
    let n = 49*3*17;
    println!("rozkład na czynniki pierwsze {n}:");
    rozklad(n);
    
    // z6
    let x = 2;
    let n = 123456789;
    let p = 1000000007;
    println!("({x}^{n})%{p} = {}", pow_mod(x, n, p));
}


fn collatz(mut n: i32) -> i32 {
    let mut steps = 0;
    while n != 1 {
        n = if n%2 == 0 { n/2 } else { 3*n+1 };
        steps += 1;
    }
    steps
}

fn armstrong(x: i32) -> bool {
    let n = x.ilog10()+1;
    let mut tmp = x;
    let mut sum = 0;
    while tmp > 0 {
        sum += (tmp%10).pow(n);
        tmp /= 10;
    }
    sum == x
}

fn doskonala(n: u32) -> bool {
    let mut sum = 0;
    for i in 1..n {
        if n%i == 0 {
            sum += i;
        }
    }
    sum == n
}

fn rozklad(mut n: i32) {
    for i in 2..=n {
        while n % i == 0 {
            print!("{i} ");
            n /= i;
        }
    }
    print!("\n");
}

fn pow_mod(mut x: u128, mut n: u128, p: u128) -> u128{
    while n > 0 {
        x = (x*x) % p;
        n -= 1;
    }
    x
}
#[allow(unused)]
fn print_ascii() {
    for z in 33..=126 {
        println!("{}: {}", z as u8 as char, z);
    }  
}