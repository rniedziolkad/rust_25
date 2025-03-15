fn main() {
    let year = 2024;

    if (year%4 == 0 && year%100 != 0) || year%400 == 0 {
        println!("Przestępny rok. Zatrzymać go!");
    } else {
        println!("Nie jest przestępny.");
    }

    // z3 i z4
    let cel = 13;
    let fahrenheit = 32.0 + 9.0/5.0 * (cel as f32);
    println!("{cel}C => {fahrenheit}F");

    let cel = (fahrenheit - 32.0) * 5.0/9.0;
    println!("{fahrenheit}F => {cel}C");

    // z5
    let h1 = 18;
    let m1 = 20;
    let s1 = 55;

    let h2 = 12;
    let m2 = 40;
    let s2 = 40;

    let t1s = h1*3600 + m1*60 + s1;
    let t2s = h2*3600 + m2*60 + s2;

    // let x = (let z = 5); nie możemy tak zrobić
    let y = {
        let x =3;
        x + 1
    };
    println!("y={y}");

    let tdiff = if t1s > t2s { t1s-t2s } else { t2s-t1s };

    let hdiff = tdiff/3600;
    let mdiff = (tdiff%3600) / 60;
    let sdiff = tdiff % 60;

    println!("Diff = {hdiff}:{mdiff}:{sdiff}");

    // pętle ----------------
    let mut i = 0;
    // pętla nieskończona
    loop {
        if i > 15 {
            break;      // przerwanie pętli
        }
        println!("Hi {i}");
        i += 1;
    }

    // przypisanie wartosci obliczonej w petli
    let result = loop {
        i += 1;
        if i==20 {
            break i*2;
        }
    };
    println!("result = {result}");
    
    let mut count = 0;
    // nazywanie pętli
    'counting_loop: loop {
        println!("count = {count}");
        let mut rem = 10;
        loop {
            println!("rem = {rem}");
            if rem == 9  {
                break;
            }
            if count == 2 {
                break 'counting_loop;   //przerywanie zewnętrznej pętli
            }
            rem -= 1;
        }
        count += 1;
    }

    // z6
    let n = 5;
    let mut i = 2;
    let mut factorial = 1;
    loop {
        if i > n {
            break;
        }
        factorial *= i;
        i += 1;
    }
    println!("{n}! = {factorial}");

    factorial = 1;
    i = 2;

    // while -- pętla z warunkiem końcowym
    while i <= n {
        factorial *= i;
        i += 1;
    }
    println!("{n}! = {factorial}");

    let range = 2..10;
    println!("{:?}", range);

    factorial = 1;
    // for -- dla każdego elementu kolekcji (w tym przypadku Range)
    for i in 2..n+1 {
        factorial *= i;
    }
    println!("{n}! = {factorial}");

    for x in [6, 11, -8, 4] {
        print!("{x}, ");
    }
    print!("\n");

    // z7 i z8

    let mut d = 12345;
    let mut sum = 0;
    while d > 0 {
        print!("{} ", d%10);
        sum += d%10;
        d /= 10;     
    }
    println!(" suma={sum}");

    let dana = 15;
    for a in 1..dana+1 {
        for b in a+1..dana+1 {
            for c in b+1..dana+1 {
                if a*a + b*b == c*c {
                    println!("trojka: {a} {b} {c}");
                }
            }
        }
    }
    // funkcje wprowadzenie
    my_fun();
    fun_with_param(5, 'c');
    let x = 7;
    let x = plus_one(x);
    println!("x = {x}");

}

fn my_fun(){
    println!("Inna funkcja");
}

fn fun_with_param(val: i32, label: char) {
    println!("val = {val}, label = {label}");
}
// po -> jest typ zwracany
fn plus_one(a: i32) -> i32 {
    let z = 1;
    // return a + z;
    // mozemy zwrócić przez wyrażenie na końcu ewaluujące się do i32
    a + z
}

