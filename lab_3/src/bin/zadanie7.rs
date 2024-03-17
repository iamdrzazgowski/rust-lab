fn pow_mod(x: u128, n: u128, p: u128) -> u128 {

    //  (x^n) % p = (x % p) ^ n % p

    if p == 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = x % p;
    let mut exp = n;

    while exp > 0 {

        if exp % 2 == 1 {
            result = (result * base) % p;
        }

        base = (base * base) % p;
        exp /= 2;
    }

    result
}

fn main(){

    let x: u128 = 12345678901234567890;
    let n: u128 = 98765432109876543210;
    let p: u128 = 1000000007;

    let result: u128 = pow_mod(x, n, p);

    println!("({}, {}) % {} = {}", x, n, p, result);
}