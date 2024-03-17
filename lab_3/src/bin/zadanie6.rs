fn prime_factors(mut n: i32) {
    let mut divisor = 2;

    while divisor <= n {
        if n % divisor == 0 {
            print!("{} ", divisor);
            n /= divisor;

        }else {
            divisor += 1;
        }
    }
}

fn main(){
    prime_factors(120);
}