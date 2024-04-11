fn nowy_napis(napis: &String) -> String{

    let mut result = String::new();
    let mut i = 0;

    while i < napis.len(){
        let litera = napis.chars().nth(i).unwrap();
        result.push_str(litera.to_string().as_str());
        i += 2;
    }

    return result;
}

fn main(){

    let napis = String::from("mleko kokosowe");
    let result: String = nowy_napis(&napis);
    println!("{}", result); // Should print "mek oosw
}