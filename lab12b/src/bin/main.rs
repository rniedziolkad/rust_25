use frac_lib::ulamek::Ulamek;

fn main() {
    let mut u1 = Ulamek::new(1, 2);
    let u2 = Ulamek::new(6, 8);
    println!("{u2}");
    dbg!(u1);
    dbg!(u2);
    dbg!(-u1);
    dbg!(u1 + u2);
    dbg!(u2 - u1);
    dbg!(u1 * u2);
    dbg!(u2 / u1);

    u1 += u2;
    dbg!(u1);
    u1 -= u2;
    dbg!(u1);

    u1 *= u2;
    dbg!(u1);
    u1 /= u2;
    dbg!(u1);

    dbg!(u1 == Ulamek::new(-4, -8));
    dbg!(u1 != u2);
}
    
