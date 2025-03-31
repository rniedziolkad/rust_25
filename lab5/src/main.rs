fn main() {
    let imie = "łukasz";
    let nazwisko = "ŚWIętOCHOWSKI";
    let w = wizytowka(imie, nazwisko);
    println!("{w}");
    let added = dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298");
    println!("{added}");
    println!("{}", dodaj_pisemnie("100", "100"));

    println!("{}", met_newt(8.0, 0.0001, 100));
    
}

fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let mut res = String::new();
    res.push(imie.to_uppercase().chars().nth(0).unwrap());
    res.push_str(". ");
    res.push(nazwisko.to_uppercase().chars().nth(0).unwrap());
    res.push_str(nazwisko
        .chars()
        .skip(1)
        .collect::<String>()
        .to_lowercase()
        .as_str()
    );
    res
}



fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut res = String::new();
    let mut i1 = a.chars().rev().peekable();
    let mut i2 = b.chars().rev().peekable();
    let mut moved = 0;
    while i1.peek() != None || i2.peek() != None || moved != 0 {  
        let va = i1.next().unwrap_or('0')
                    .to_digit(10).unwrap_or(0);
        let vb = i2.next().unwrap_or('0')
                    .to_digit(10).unwrap();

        let sum = va+vb+moved;
        moved = sum / 10;
        let ch = char::from_digit(sum%10, 10).unwrap();
        res.push(ch);
    }
    res.chars().rev().collect()
}

fn f(x: f64) -> f64 {
    x*x - 4.0
}

fn fp(x: f64) -> f64 {
    2.0*x
}

fn met_newt(mut x0: f64, eps: f64, n: u32) -> f64{
    let mut next = x0 - (f(x0)/fp(x0));
    // wyznaczanie kolejnych wartości w pętli
    let mut i = 0u32;
    while (next - x0).abs() > eps && i < n {
        x0 = next;
        next = x0 - (f(x0)/fp(x0));
        i += 1;
    }
    x0
}
