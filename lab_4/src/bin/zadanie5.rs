fn sort(a: &mut u32, b: &mut u32, c: &mut u32){
    if b < a{
        let temp = *a;
        *a = *b;
        *b = temp;
    }

    if c < b {
        let temp = *c;
        *c = *b;
        *b = temp;
    }

    if b < a{
        let temp = *b;
        *b = *a;
        *a = temp;
    }
}

fn main(){
    let mut a = 7;
    let mut b = 2;
    let mut c = 3;
    sort(&mut a, &mut b, &mut c);
    println!("{}, {}, {}", a, b, c);
}