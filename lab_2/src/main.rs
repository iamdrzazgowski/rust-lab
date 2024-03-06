fn main() {
    
    // Zadanie 1 (Silnia)

    let n = 5;
    let mut silnia = 1;

    for i in 1..n+1{
        silnia *= i;
    }

    println!("Silnia z {} to {}", n, silnia);

    // Zadanie 2 i 3 (Suma cyfr)

    let mut num = 134;
    let mut sum = 0;
    
    while num > 0 {
        
        println!("{}", num % 10);
        sum += num % 10;
        num /= 10;
    }

    println!("Suma cyfr to {}", sum);
    
}
