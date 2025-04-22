fn main() {
    let mut squares: Vec<i32> = Vec::new();
    for i in 1..=10 {
        squares.push(i*i);
    }
    println!("{squares:?}");
    // z1
    // a)
    let _letters: Vec<char> = ('a'..='z').collect();
    let letters = ('a'..='z').collect::<Vec<_>>();
    println!("{letters:?}");
    // b)
    let closure1 = |x: i32| x*x;
    let _squares = (1..=10).map(closure1).collect::<Vec<_>>();  // przekształcenie (map)
    let _squares: Vec<i32> = (1..=10).map(|x| x*x).collect();
    let squares: Vec<u32> = (1..=10).map(square).collect();
    println!("{squares:?}");

    // c)
    let powers2: Vec<_> = (1..=10).map(|x| 2_i32.pow(x)).collect();
    println!("{powers2:?}");

    //d)
    let odwrotnosc: Vec<_> = (1..=20).map(|x| 1.0/x as f64).collect();
    println!("{odwrotnosc:?}");

    let _filtered: Vec<_> = (1..=100).filter(|x| x%3==0 && x%4!=0).collect();
    let closure2 = |x: &i32| -> bool {
        return x%3==0 && x%4!=0;
    };
    let filtered: Vec<_> = (1..=100).filter(closure2).collect();
    println!("{filtered:?}");

    //z2
    let examples = vec![
        "Ala".to_string(), "kot".to_string(), "koń".to_string(),
        "2 domki".to_string(), "długi napis".to_string(), "123".to_string(),
        "pizza".to_string(), "lekki".to_string(), "123".to_string()
    ];
    for e in examples.windows(3){
        println!("{e:?}");
    }


    println!("{:?}", filter_a(&examples));
    println!("{:?}", filter_b(&examples));
    println!("{:?}", filter_c(&examples));
    println!("{:?}", map_d(&examples));
    println!("{:?}", filter_e(&examples));

    println!("\"123\" indeksy: {:?}", indeksy(&examples, "123"))
}
fn indeksy(tablica: &Vec<String>, element: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();
    for (i, e) in tablica.iter().enumerate(){
        if e == element {
            indices.push(i);
        }
    }
    indices
}
fn filter_e(arr: &Vec<String>) -> Vec<&String> {
    arr.iter()
        .filter(
            |s| s.chars().collect::<Vec<char>>()
                .windows(2)
                .any(|pair| pair[0]==pair[1])
        ).collect()
}

fn map_d(arr: &Vec<String>) -> Vec<String> {
    arr.iter().map(|s| s.chars().rev().collect::<String>())
        .collect()
}

fn filter_c(arr: &Vec<String>) -> Vec<&String> {
    // arr.iter()
    //     .filter(|s| s.contains(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']))
    //     .collect()
    arr.iter()
        .filter(|s| s.chars().any(|c| c.is_digit(10)))
        .collect()
}

fn filter_b(arr: &Vec<String>) -> Vec<&String> {
    arr.iter()
        .filter(|s| !s.contains('a') && !s.contains('A'))
        .collect()
}

fn filter_a(arr: &Vec<String>) -> Vec<String> {
    arr.iter().filter(|s| s.chars().count() < 4)
        .cloned()
        .collect()
}

fn square(x: i32) -> u32 {
    (x*x) as u32
}

