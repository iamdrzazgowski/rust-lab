fn perfect_number(n: i32) -> bool {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0{
            sum += i;
        }
    }
    sum == n
}

fn main(){
    let number = 28;
    println!("Liczba {} : {}", number, perfect_number(number));
}