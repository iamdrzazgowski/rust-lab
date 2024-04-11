fn count_letter(text: &str, letter: char) -> usize{
    let lower_case_text = text.to_lowercase();
    return lower_case_text.chars().filter(|&c| c == letter).count();
}

fn main(){
    let text = "Ala ma kota";
    let letter = 'a';
    
    println!("Number of letter {} in text: {}", letter, count_letter(text, letter));
}