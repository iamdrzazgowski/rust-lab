fn position(element: i32, array: &[i32]) -> Option<usize>{
    for i in 0..array.len(){
        if array[i] == element{
            return Some(i);
        }
    }

    None
}

fn main(){

    let array: [i32; 4] = [1,2,3,4];
    let mut position_v = position(2, &array);
    println!("{:?}", position_v);

    position_v = position(6, &array);
    println!("{:?}", position_v);

    if let Some(t) = position_v {
        println!("Pozycja w tablicy: {}", t);
    }else{
        println!("Elementu nie ma w tablicy");
    }
}