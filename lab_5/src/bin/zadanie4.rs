fn reverse_text(napis: &String) -> String{
    napis.chars().rev().collect()
}

fn main(){
    let napis = String::from("Ala ma kota");

    let result: String = reverse_text(&napis);
    println!("{}", result);
}