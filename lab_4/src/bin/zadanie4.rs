fn avg(arr: &[u32]) -> f32 {
    let mut sum = 0;
    for i in 0..arr.len(){
        sum += arr[i];
    }

    sum as f32 / arr.len() as f32
}


fn main(){
    let arr = [1,2,3,4,5,6];
    let average = avg(&arr);
    println!("Średnia wynosi: {}", average)
}