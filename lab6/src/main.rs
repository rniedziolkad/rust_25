fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }
    let values = ["000", "001", "010", "011",
                            "100", "101", "110", "111"];
    let mut res = String::new();
    for c in z.chars(){
        let value = c.to_digit(8)? as usize;
        let value = values[value]; 
        //let value = &format!("{value:b}"); <- daje wartość binarną liczby, ale najkrótszą co jest tu problemem (chcemy 3 znakową)
        res += value;
    }
    Some(res.trim_start_matches('0').to_string())
}

fn wartosc_syst2(z: &str) -> Option<u8> {
    if z.is_empty() || z.len() > 8 {
        return None;
    }
    let mut res = 0u8;
    for c in z.chars() {
        res <<= 1;      // res *= 2;
        res += c.to_digit(2)? as u8;
    }
    Some(res)
    // u8::from_str_radix(z, 8).ok()
}

fn wartosc_syst8_v1(z: &str) -> Option<u8> {
    let binary = zamien_syst8_na_syst2(z);
    if binary.is_none() {
        return None;
    }
    wartosc_syst2(&binary.unwrap())
}

fn wartosc_syst8_v2(z: &str) -> Option<u8> {
    wartosc_syst2(zamien_syst8_na_syst2(z)?.as_str())
}

fn main() {
    let x = "24705";
    let err = "129";
    let empty = "";
    println!("{x} = {:?}", zamien_syst8_na_syst2(x));
    println!("{err} = {:?}", zamien_syst8_na_syst2(err));
    println!("{empty} = {:?}", zamien_syst8_na_syst2(empty));
    println!();

    let b = "11111111";
    let err = "102";
    let empty = "";
    let long = "100000000";
    println!("{b} = {:?}", wartosc_syst2(b));
    println!("{err} = {:?}", wartosc_syst2(err));
    println!("{empty} = {:?}", wartosc_syst2(empty));
    println!("{long} = {:?}", wartosc_syst2(long));
    println!();

    let o = "377";
    let err = "38";
    let empty = "";
    let long = "400";
    println!("{o} = {:?}", wartosc_syst8_v1(o));
    println!("{err} = {:?}", wartosc_syst8_v1(err));
    println!("{empty} = {:?}", wartosc_syst8_v1(empty));
    println!("{long} = {:?}", wartosc_syst8_v1(long));
    println!();

    println!("{o} = {:?}", wartosc_syst8_v2(o));
    println!("{err} = {:?}", wartosc_syst8_v2(err));
    println!("{empty} = {:?}", wartosc_syst8_v2(empty));
    println!("{long} = {:?}", wartosc_syst8_v2(long));
    println!();

    // 5b
    let c = '9';
    let err = 'a';
    println!("{c} = {:?}", wartosc_cyfry(c));
    println!("{err} = {:?}", wartosc_cyfry(err));
    println!();

    let a = "9999991";
    let b = "9";
    let err = "12L";
    let empty = "";
    println!("{a} + {b} = {:?}", dodaj_pisemnie(a, b));
    println!("{a} + {err} = {:?}", dodaj_pisemnie(a, err));
    println!("{a} + {empty} = {:?}", dodaj_pisemnie(a, empty));

}

fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    if a.is_empty() || b.is_empty() {
        return Err("Pusta liczba".to_string());
    }
    let mut res = String::new();
    let mut i1 = a.chars().rev().peekable();
    let mut i2 = b.chars().rev().peekable();
    let mut moved = 0;
    while i1.peek() != None || i2.peek() != None || moved != 0 {
        let va = wartosc_cyfry(i1.next().unwrap_or('0'))?;
        let vb = wartosc_cyfry(i2.next().unwrap_or('0'))?;

        let sum = va+vb+moved;
        moved = sum/10;
        let ch = (sum%10) + '0' as u8;
        res.push(ch as char);
    }
    Ok(res.chars().rev().collect())
}

fn wartosc_cyfry(c: char) -> Result<u8, String> {
    let value = c.to_digit(10);
    if value.is_none() {
        return Err(format!("'{c}' nie jest cyfrą"));
    }
    Ok(value.unwrap() as u8)
}
