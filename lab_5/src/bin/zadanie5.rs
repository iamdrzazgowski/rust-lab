fn szyfruj(napis: &String, klucz: usize) -> String{
    let mut result: String = String::new();
    let it = napis.chars();

    let mut offset: usize = 0;

    loop{
        if offset * klucz > napis.len(){
            break;
        }

        result += it.clone().skip(offset * klucz).take(klucz).collect::<String>().chars().rev().collect::<String>().as_str();

        offset += 1;
    }

    result
}

fn main(){

    let napis = String::from("kaszanka");
    let result = szyfruj(&napis, 3);
    println!("Zaszyfrowany napis: {}", result);
}