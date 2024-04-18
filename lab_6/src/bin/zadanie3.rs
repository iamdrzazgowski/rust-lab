fn divisors(number: Option<u32>) -> usize{
    let value = number.unwrap();

    let mut divider = 0;

    for i in 1..=value{
        if value % i == 0 {
            divider += 1;
        }
    }

    divider
}

fn main(){
    let number = 7;
    println!("{}", divisors(Some(number)));
    // println!("{}", divisors(None));
}