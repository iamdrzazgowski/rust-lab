fn collatz_steps(n: &mut i32){
    *n += 1;
}

fn main(){
    // let steps = 12;
    let mut current_step = 0;
    collatz_steps(&mut current_step);
    println!("Current step: {}", current_step);
}