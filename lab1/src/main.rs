#[allow(unused)]
fn main() {
    println!("Hello, world!");
    let x;
    x = 6;
    println!("x = {} {} {}", x, x, x);
    println!("x = {x}");

    const SECONDS_IN_HOUR: i32 = 60*60;
    println!("{SECONDS_IN_HOUR}");

    let x = x+1;
    {
        let x = x*2;
        println!("W środku x= {x}");
    }
    println!("x = {x}");

    let spaces = "    ";
    let spaces = spaces.len();      //zmienna przesłania i zmienia typ
    println!("spaces = {spaces}");
    /*
    *   skalarne:
    *       całkowite:
    *           - i8 u8 -> 8 bitowa liczba całkowita ze znakiem/bez znaku
    *           - i16 u16
    *           - i32 u32
    *           - i64 u64
    *           - i128 u128
    *           - isize usize -> zależnie od architektury 32 bity lub 64 bity
    *   
    *       zmiennoprzecinkowe:
    *           - f32 f64
    *   bool
    *   char
     */
    let a = 2;
    let b = 2.0;
    let c: u64 = 3;
    let c = 3u16;
    let c: f32 = 2f32;         //jawnie zdefinowane jako zmiennoprzecinkowe
    let c = 10_000;
    let c = 0xff;
    let c = 0o77;
    let c = 0b1111_0000;
    let c = b'A';

    let c = 'A';
    
    let c: usize = 7;

    let a = 11;
    let b = 5;
    let c = a+b;
    println!("{c}");
    let c = a*b;
    let c = a-b;
    let c = a/b;
    println!("/ = {c}");
    let d = 5.0;
    let c = 11.0/d;
    println!("dzielenie zmiennoprzecinkowe = {c}");
    println!("12.0 / 3.0 = {}", 12.0/3.0);
    println!("a/b = {}", a/b);

    let a = 7;
    let b = 7;
    //instrukcje warunkowe
    if a < b || a == b {
        println!("a <= b");
    } else {
        println!("a > b");
    }

    let mut x = 255u8;
    x += 1;                 // działa w trybie --release (dojdzie do przepełnienia i wróci do 0)
    println!("x = {x}");

}
