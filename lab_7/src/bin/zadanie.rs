fn krotsze_niz(napisy: Vec<&str>, dlugosc: usize) -> Vec<&str> {
    napisy.iter().filter(|s| s.len() < dlugosc).map(|x| *x).collect()
}

// fn nie_zawiera(napisy: Vec<&str>) -> Vec<&str> {
//     napisy.iter().map(|x| 
// }

fn indeksy(wektor: Vec<u32>, element: u32) -> Vec<usize> {
    wektor.iter().zip(0..=wektor.len()).filter(|(x, _)| **x == element).map(|(_, id)| id).collect()
}

fn main(){

    // alfabet od a do z
    let litery: Vec<_> = ('a'..='z').collect();
    println!("{:?}", litery);

    // Kwadrat 10 kolejnych liczb
    let power: Vec<_> = (1..=10).map(|x| x*x).collect();
    println!("{:?}", power);

    // Kolejne potęgi 2
    let power2: Vec<_> = (1..=10).map(|x| u32::pow(2,x)).collect();
    println!("{:?}", power2);

    // Podzielne przez 3 i nie podzielne przez 4
    let divided: Vec<_> = (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect();
    println!("{:?}", divided);

    // Zadanie 9

    let napisy = vec!["kot", "pies", "ala", "mleko", "aleksander wielki"];
    println!("{:?}", krotsze_niz(napisy, 4));
    // println!("{:?}", nie_zawiera(napisy));

    let liczby = vec![1,2,3,4,4,5,6,3,6];

    println!("{:?}", indeksy(liczby, 5))

}