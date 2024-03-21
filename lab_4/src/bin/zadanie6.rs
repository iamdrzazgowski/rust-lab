fn swap_range(array: &mut [u32], f1: (usize, usize), f2: (usize, usize)){

    let mut range1 = f1;
    let mut range2 = f2;
    
    let mut offset = 0;

    let len1 = range1.1 - range1.0;
    let len2 = range2.1 - range2.0;

    if len1 > len2 {
        range1.1 = range1.0 + len2;
    }else if len2 > len1 {
        range2.1 = range2.0 + len1;
    }

    for i in range1.0..=range1.1{
        let temp = array[range2.0 + offset];
        array[range2.0 + offset] = array[i];
        array[i] = temp;
        offset += 1;
    }
}

fn main(){
    let mut array = [0,1,2,3,4,5,6,7,8,9];
    let range1 = (1,3);
    let range2 = (5,7);

    println!("{:?}", array);
    swap_range(&mut array, range1, range2);
    println!("{:?}", array);
}