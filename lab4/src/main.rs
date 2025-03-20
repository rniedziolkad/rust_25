fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn sort_three(a: &mut i32, b: &mut i32, c: &mut i32) {
    if *a > *b {
        swap(a, b);
    }
    if *b > *c {
        swap(b, c);
    }
    if *a > *b {
        swap(a, b);
    }
}

fn rand(seed: &mut u32, min_rand: i32, max_rand: i32) -> i32 {
    let a: u32 = 1664525;
    let c: u32 = 1013904223;
    *seed = a.wrapping_mul(*seed).wrapping_add(c);
    let range = (max_rand - min_rand + 1) as u32;
    ((*seed) % range) as i32 + min_rand
}

fn swap_arr(arr: &mut [i32], i: usize, j: usize) {
    // arr.swap(i, j);
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

fn rand_perm(arr: &mut [i32], seed: &mut u32) {
    for i in (1..arr.len()).rev() {
        let j: usize = rand(seed, 0, i as i32) as usize;
        swap_arr(arr, i, j);
    }
}

fn main() {
    // zestaw 3
    // z1 i z2
    let mut x = 5;
    let mut y = 10;
    let mut z = 3;
    println!("Przed sortowaniem: x = {}, y = {}, z = {}", x, y, z);
    sort_three(&mut x, &mut y, &mut z);
    println!("Po sortowaniu: x = {}, y = {}, z = {}", x, y, z);
    // z3
    let mut seed: u32 = 42;
    println!("Wygenerowane liczby pseudolosowe:");
    for _ in 0..5 {
        let r = rand(&mut seed, 1, 100);
        println!("{}", r);
    }
    // z4
    let mut arr = [1, 2, 3, 4, 5];
    println!("Tablica przed zamianą: {:?}", arr);
    swap_arr(&mut arr, 1, 3);
    println!("Tablica po zamianie: {:?}", arr);
    // z5
    rand_perm(&mut arr, &mut seed);
    println!("Tablica po permutacji: {:?}", arr);

    
    // iterator -- pozwala na iterowanie po kolekcji danych (przechodzenie przez jej elementy)
    println!("-----------------------\nIteratory:\n");
    let arr = [4, 7, 11];
    let mut it = arr.iter();
    let val = it.next();      // wartość next() jest opakowana w Option<T> 
    println!("{:?}", val.unwrap());         // gdzie Some(T) zawiera kolejny element
    println!("{:?}", it.next());            
    println!("{:?}", it.next());
    println!("{:?}", it.next());            // None oznacza, że iteracja dobiegła końca
    // enum Option<T> {                     // (enum)erator -- typ wyliczający możliwe wartości
    //      None,
    //      Some(T)
    // }

    // Vec (wektor) -- jak tablica, przechowuje jeden typ, ale może mieć zmienną długość
    let v1 = vec![1, 2, 3, 4];    // Vec::from([1,2 ,3 ,4]);
    println!("Pętla 'for':");
    let v1_iter = v1.iter();
    for v in v1_iter {                // pętla for korzysta z iteratorów
        print!("{v} ");
    }
    println!("\n-----------------------");

    // zestaw 4a
    // z1
    let s = "ala ma kota";
    println!("wystapeń 'a' w '{s}': {}", liczba_wystapien(s, 'a'));
    // z2
    let r = "MCMXXLVIII";
    println!("{r} = {}", rzymskie(r));

    // zestaw 4b
    // z1
    let s = "abcdefghi";
    let part = co_drugi_znak(s);
    dbg!(part);
    let part2 = co_drugi_znakv2(s);
    dbg!(part2);
    // z2
    println!("zaszyfrowany: {}", szyfruj("kot Mruczek", 9));
   
}

fn szyfruj(napis: &str, klucz: usize) -> String {
    let mut encoded = String::new();
    let _slice = &napis[0..3];           // slice (wycinek) -- referencja do ciągłej części kolekcji
    for i in (0..napis.len()).step_by(klucz) {
        if i+klucz < napis.len() {
            // używamy slice'ingu (świadomie; zakładamy, że nie występują znaki wielobajtowe)
            encoded += napis[i..i+klucz].chars().rev().collect::<String>().as_str();
        } else {
            encoded += napis[i..].chars().rev().collect::<String>().as_str();
        }
    }
    encoded
}

fn co_drugi_znakv2(napis: &str) -> String {
    // collect konsumuje iterator i przekształca w odpowiednią kolekcję
    napis.chars().step_by(2).collect()
}

fn co_drugi_znak(napis: &str) -> String {
    let mut out = String::new();
    for c in napis.chars().step_by(2) {
        out.push(c);
    }
    out
} 

fn rzymskie(napis: &str) -> i32 {
    let mut res = 0;
    let mut prev = 0;
    
    for c in napis.chars().rev() {
        let roman = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,             // match wymaga obsłużenia wszystkich możliwych przypadków
        };
        if roman < prev {
            res -= roman;
        } else {
            res += roman;
        }
        prev = roman;
    }
    res
}

fn liczba_wystapien(napis: &str, znak: char) -> i32 {
    let mut count = 0;
    let iterator = napis.chars();
    for c in iterator {
        if c == znak {
            count += 1;
        }
    }
    count
}