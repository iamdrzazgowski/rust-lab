fn main() {

    // Zadanie 1

    let year: i64 = 2024;

    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        println!("Rok {} jest rokiem przestępnym", year);
    } else {
        println!("Rok {} nie jest rokiem przestępnym", year);
    }

    // Zadanie 2

    let month: i64 = 2;

    if month == 2 {
        if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
            println!("29 dni");
        }else {
            println!("28 dni");
        }

    }else if month == 4 || month == 6 || month == 9 || month == 11 {
        println!("30 dni");

    }else {
        println!("31 dni");
    }

    // Zadanie 3 i 4

    let celsius: f32 = 21.0;
    let fahrenheit: f32 = 77.0;

    let celsius_to_fahrenheit: f32 = (celsius * 9.0 / 5.0) + 32.0;
    let fahrenheit_to_celsius: f32 = (fahrenheit - 32.0) * 5.0 / 9.0;
    
    println!("{} stopni Celsjusza to {} stopni Fahrenheita", celsius, celsius_to_fahrenheit);
    println!("{} stopni Fahrenheita to {} stopni Celsjusza", fahrenheit, fahrenheit_to_celsius);


    // Zadanie 5
    
    let first_hh = 15;
    let first_mm = 43;
    let first_ss = 54;

    let second_hh = 11;
    let second_mm = 23;
    let second_ss = 54;

    let first_seconds = first_hh * 3600 + first_mm * 60 + first_ss;
    let second_seconds = second_hh * 3600 + second_mm * 60 + second_ss;

    let difference = first_seconds - second_seconds;

    let diff_hh = difference / 3600;
    let diff_mm = (difference % 3600) / 60;
    let diff_ss = (difference % 3600) % 60;

    println!("Różnica czasu to {}:{}:{}", diff_hh, diff_mm, diff_ss);

}
