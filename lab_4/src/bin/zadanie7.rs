fn rand(seed: u64, min: u64, max: u64) -> u64 {
    let a = 1664525;
    let c = 1013904223;
    let m = 2_u64.pow(32);

    let mut x = seed;
    x = (a * x + c) % m;

    min + x % (max - min + 1)
}

fn main() {
    
    let seed = 45;
    let min = 0;
    let max = 1000;

    for _ in 0..10 {
        let random_number = rand(seed, min, max);
        println!("{}", random_number);
    }
}