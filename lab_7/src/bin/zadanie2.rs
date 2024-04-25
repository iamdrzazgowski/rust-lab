fn unikalne(vect: Vec<u32>) -> Vec<u32>{

    let mut out = Vec::new();

    for element in vect {
        if !out.contains(&element){
            out.push(element);
        }
    }

    out
}

fn no_occurences(v: &Vec<u32>, e: u32) -> usize {
    let wektor: Vec<u32> = v.iter().filter(|x| **x == e).map(|x| *x).collect();

    wektor.len()
}

fn unikalne_lepiej(v: Vec<u32>) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();

    for element in &v {
        if no_occurences(&v, *element) == 1{
            out.push(*element);
        }
    }
    
    out
}

fn main(){
    let vect: Vec<_> = vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6];  
    println!("{:?}", unikalne(vect.clone()));
    println!("{:?}", unikalne_lepiej(vect));

}