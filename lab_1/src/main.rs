fn main() {

    // Zadanie 1

    let year: i64 = 2023;

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


}
