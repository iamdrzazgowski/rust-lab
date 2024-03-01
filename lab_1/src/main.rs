fn main() {
    let year: i64 = 900;

    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
        println!("Rok {} jest rokiem przestępnym", year);
    } else {
        println!("Rok {} nie jest rokiem przestępnym", year);
    }
}
