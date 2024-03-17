fn collatz_steps(n: i32, iteration: i32) -> i32 {

    if n == 1 {
        return iteration;
    }else if n % 2 == 0{
        return collatz_steps(n / 2, iteration + 1);
    }else{
        return collatz_steps(3 * n + 1, iteration + 1);
    }
    
}

fn main(){
    let steps = 12;
    println!("Wynik: {}",collatz_steps(steps, 0));
}