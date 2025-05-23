#[derive(Debug, PartialEq, PartialOrd)]
enum CardSuit {
    Trefl,
    Karo,
    Kier,
    Pik,
}

enum FileError {
    None,
    InvalidFormat,
    NotFound{filename: String},
    TooLarge{current_size: u64, max_size: u64},
}

impl FileError {
    fn show_message(&self) {
        match self {
            FileError::None => println!("Brak błędu"),
            FileError::InvalidFormat => println!("Nieprawidłowy format pliku"),
            FileError::NotFound{filename} => println!("Plik '{filename}' nie istnieje"),
            FileError::TooLarge{ current_size, max_size } => {
                println!("Plik za duży: {current_size} B (maksymalny rozmiar pliku to {max_size} B)");
            }
        }
    }
}

fn main() {
    let pik = CardSuit::Pik;
    let kier = CardSuit::Kier;
    let karo = CardSuit::Karo;
    let trefl = CardSuit::Trefl;

    println!("{}", pik > kier);
    println!("{}", kier > karo);
    println!("{}", karo > trefl);

    let no_error = FileError::None;
    no_error.show_message();
    let invalid_format = FileError::InvalidFormat;
    invalid_format.show_message();
    let not_found = FileError::NotFound{ filename: "test.file".to_string() };
    not_found.show_message();
    let too_large = FileError::TooLarge{ current_size: 121, max_size: 100 };
    too_large.show_message();


}