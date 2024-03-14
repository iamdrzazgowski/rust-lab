fn ascii_table(){
    for i in 33..127{
        println!("{} - {}", i as u8 as char, i);
    }
}

fn main(){
    ascii_table();
}