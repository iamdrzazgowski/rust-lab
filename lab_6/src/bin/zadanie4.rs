 fn wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String{

    let imie_v;
    let nazwisko_v;

    // let imie_v = imie.unwrap_or(String::from("Jan"));
    // let nazwisko_v = nazwisko.unwrap_or(String::from("Kowalski"));
    
    if let Some(y) = imie{
        imie_v = y;
    }else{
        imie_v = String::from("Jan");
    }

    if let Some(x) = nazwisko{
        nazwisko_v = x;
    }else{
        nazwisko_v = String::from("Kowalski");
    }

    let mut result = String::new();

    let first_letter = imie_v.to_uppercase().chars().nth(0).unwrap();
    
    let nazwisko_upper = nazwisko_v.to_uppercase().chars().nth(0).unwrap();

    let mut i = 1;

    result.push_str(first_letter.to_string().as_str());
    result.push_str(". ");
    result.push_str(nazwisko_upper.to_string().as_str());

    while i < nazwisko_v.len(){
        let litera = nazwisko_v.to_lowercase().chars().nth(i).unwrap();
        result.push_str(litera.to_string().as_str());
        i += 1;
    }

    result

 }

fn main(){

    let imie = Some(String::from("Adrian"));
    let nazwisko = None;
    let wizytowka_v = wizytowka(imie, nazwisko);
    println!("{}", wizytowka_v);
}