fn fraction(numerator: i32, denominator: i32) -> Option<f32> {
    let numerator_f = numerator as f32;
    let denominator_f = denominator as f32;

    if denominator != 0{
        Some(numerator_f / denominator_f)

    }else{
        None
    }
}

fn main(){
    
    println!("{:?}", fraction(1,2));
    println!("{:?}", fraction(1,0));

    let result = fraction(1,2);
    println!("{}", result.unwrap() + 2.0);

    let result = fraction(1,0);
    // println!("{}", result.unwrap() + 2.0);
    println!("{}", result.expect("tu jest None") + 2.0);
}