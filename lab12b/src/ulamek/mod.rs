#[cfg(test)]
mod tests;

use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ulamek {
    licznik: i64,
    mianownik: i64,
}

impl Ulamek {
    pub fn new(licznik: i64, mianownik: i64) -> Self {
        if mianownik == 0 {
            panic!("Mianownik nie może być 0!");
        }
        let mut licznik = licznik;
        let mut mianownik = mianownik;

        if mianownik < 0 {
            licznik = -licznik;
            mianownik = -mianownik;
        }

        let nwd = Self::nwd(licznik, mianownik);
        Self { licznik: licznik / nwd, mianownik: mianownik / nwd }
    }

    pub fn licznik(&self) -> i64 {
        self.licznik
    }

    pub fn mianownik(&self) -> i64 {
        self.mianownik
    }

    pub fn as_f64(&self) -> f64 {
        self.licznik as f64 / self.mianownik as f64
    }

    fn nwd(a: i64, b: i64) -> i64 {
        let mut a = a.abs();
        let mut b = b.abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn from_str(napis: &str) -> Option<Self> {
        let parts: Vec<&str> = napis.split("/").collect(); 
        if parts.len() == 1 {
            let licznik: i64 = parts[0].parse().unwrap();
            return Some(Self::new(licznik, 1));
        } else if parts.len() == 2 {
            let licznik: i64 = parts[0].parse().expect("Niepoprawny licznik!");
            let mianownik: i64 = parts[1].parse().expect("Niepoprawny mianownik!");
            return Some(Self::new(licznik, mianownik));
        }

        panic!("Nieprawidłowy format ułamka!");
    }
}


impl Add for Ulamek {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.licznik * rhs.mianownik + rhs.licznik * self.mianownik,
            self.mianownik * rhs.mianownik
        )
    }
}

impl Neg for Ulamek {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Ulamek::new(-self.licznik, self.mianownik)
    }
}

impl Sub for Ulamek {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Ulamek {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Ulamek::new(
            self.licznik * rhs.licznik, 
            self.mianownik * rhs.mianownik
        )
    }
}

impl Div for Ulamek {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Ulamek::new(
            self.licznik * rhs.mianownik, 
            self.mianownik * rhs.licznik
        )
    }
}


impl AddAssign for Ulamek {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Ulamek {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Ulamek {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Ulamek {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}


impl std::fmt::Display for Ulamek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.mianownik == 1 {
            write!(f, "{}", self.licznik)
        } else {
            write!(f, "{}/{}", self.licznik, self.mianownik)
        }
    }
}

