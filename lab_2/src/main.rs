fn main() {
    
    // Zadanie 1 (Silnia)

    let n = 5;
    let mut silnia = 1;

    for i in 1..n+1{
        silnia *= i;
    }

    println!("Silnia z {} to {}", n, silnia);
}