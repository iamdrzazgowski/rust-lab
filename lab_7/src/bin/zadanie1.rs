fn powtorki(v: Vec<u32>) -> Vec<u32>{
    let mut out = Vec::new(); // pusty vektor

    let mut last_pushed = false;

    for i in 1..v.len(){
        if v[i] == v[i-1] {
            out.push(v[i]);
            last_pushed = true;
        }else if last_pushed {
            out.push(v[i-1]);
            last_pushed = false;
        }
    }

    out
}

fn main(){
    let wektor: Vec<_> = vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6];
    println!("{:?}", powtorki(wektor))
}