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

// Metoda Newtona funkcje

fn f(x: f64) -> f64 {
    3.0 * x - 9.0
}

fn signum(x: f64) -> i8{
    let epsilon = 0.0000001;
    if x.abs() < epsilon {
        0
    }else if x > epsilon{
        1
    }else {
        -1
    }
}

fn sgn_f_deriv(x: f64) -> i8 {
    let epsilon = 0.0000001;
    let d = f(x + epsilon) - f(x);
    signum(d)
}

fn newton(n: u64) -> f64{
    let mut x = 7.0;
    let mut delta = 1.0;

    let mut prvs_jump_left = false;
    for _i in 0..n{
        let f_val = f(x);
        let deriv_sng = sgn_f_deriv(x);

        println!("f = {}", f_val);
        println!("f' = {}", deriv_sng);
        println!("x = {}", x);
        println!("delta = {} \n", delta);

        let mut jump_left: bool = false;

        if (signum(f_val) > 0 && deriv_sng < 0) || (signum(f_val) < 0 && deriv_sng > 0){
            x += delta;
        }else{
            x -= delta;
            jump_left = true;
        }

        if jump_left != prvs_jump_left{
            delta /= 2.0;
        }

        prvs_jump_left = jump_left;
    }

    x
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

    println!("\nZadanie 5 (Metoda Newtona)"); // Do zrobienia na następne zajęcia wymyślić (a,b, epsilon i delta)

    newton(1000);
    
}
