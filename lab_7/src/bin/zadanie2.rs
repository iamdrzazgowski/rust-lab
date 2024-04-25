fn unikalne(vect: Vec<u32>) -> Vec<u32>{

    let mut out = Vec::new();

    for element in vect {
        if !out.contains(&element){
            out.push(element);
        }
    }

    out
}

fn unikalne_lepiej(v: Vec<u32>) -> Vec<u32> {
    let mut out = Vec::new();
    
    for i in 0..v.len() {
        let mut counter = 0;

        for j in 0..v.len(){
            if v[i] == v[j] {
                counter += 1;
            }
        }

        if counter - 1 == 0 {
            out.push(v[i]);
        }
    }

    out
}

fn main(){
    let vect: Vec<_> = vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6];  
    println!("{:?}", unikalne(vect.clone()));
    println!("{:?}", unikalne_lepiej(vect));

}