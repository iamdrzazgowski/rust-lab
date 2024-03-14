fn armstrong(x: usize){
    let mut digits_counter = 0; 
    let mut number = x;
    let mut mod_number = x;
    let mut sum = 0;

    while number > 0{
        digits_counter += 1;
        number /= 10; 
    }

    while mod_number > 0{
        sum += (mod_number % 10).pow(digits_counter);
        mod_number /= 10;
    }

    if sum == x{
        println!("Liczba {} jest liczbą Armstronga", x);
    }else {
        println!("Liczba {} nie jest liczbą Armstronga", x);
    }
}

fn main(){
    let number = 408;
    armstrong(number);
}