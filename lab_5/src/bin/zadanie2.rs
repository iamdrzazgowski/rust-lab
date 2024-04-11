fn wizytowka(imie: &String, nazwisko: &String) -> String{

    let mut result = String::new();
    
    let first_letter = imie.to_uppercase().chars().nth(0).unwrap();
    let nazwisko_upper = nazwisko.to_uppercase().chars().nth(0).unwrap();

    let mut i = 1;

    result.push_str(first_letter.to_string().as_str());
    result.push_str(". ");
    result.push_str(nazwisko_upper.to_string().as_str());

    while i < nazwisko.len(){
        let litera = nazwisko.to_lowercase().chars().nth(i).unwrap();
        result.push_str(litera.to_string().as_str());
        i += 1;
    }

    return result;
}

fn main(){
    let imie = String::from("jan");
    let nazwisko = String::from("KOWALSKI");
    let result: String = wizytowka(&imie, &nazwisko);
    println!("{}", result);
}