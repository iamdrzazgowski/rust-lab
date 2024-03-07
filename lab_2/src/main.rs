fn silnia(n :usize) -> usize {
    let mut silnia = 1;

    for i in 1..=n{
        silnia *= i;
    }

    return silnia;
}

fn suma_cyfr(n: usize) -> usize {
    let mut num = n;

    let mut sum = 0;


    print!("Cyfry liczby {} to: ", n);
    while num != 0 {
        
        print!("{} ", num % 10);
        sum += num % 10;
        num /= 10;
    }

    print!("\n");

    return sum;
}

fn pitagoras(max_number: usize){
    for a in 1..=max_number{
        for b  in a+1..=max_number{
            for c in b+1..=max_number {
                if a*a + b*b == c*c{
                    println!("{}^2 + {}^2 = {}^2", a, b, c);
                }   
            }
        }
    }
}

fn main() {
    
    
    // Zadanie 1 (Silnia)
    println!("Zadanie 1 (Silnia)"); 

    let n = 5;
    println!("Silnia z {} to {} \n", n, silnia(n));

    // Zadanie 2 i 3 (Suma cyfr)
    println!("Zadanie 2 i 3 (Suma cyfr)");

    let number = 134;

    println!("Suma cyfr z liczby {} to {} \n", number, suma_cyfr(number));

    
    // Zadanie 4 (trójki pitagorejskie) 0 < a < b < c.
    println!("Zadanie 4 (trójki pitagorejskie)");

    let max_number = 10;

    pitagoras(max_number);

    // Zadanie 5 (Metoda Newtona)

    println!("\nZadanie 5 (Metoda Newtona)"); // Do zrobienia na następne zajęcia

    
}
