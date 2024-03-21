fn fill_array(){
    const N: usize = 30;
    let mut remainders: [u32; N] = [0; N];

    for i in 1..=N{
        remainders[i - 1] = 100 % i as u32;
    }

    // remainders.reverse();
    // println!("{:?}", remainders);

    for i in 1..=N{
        print!("{} ",  remainders[N - i]);
    }
}

fn main(){
    fill_array();
}