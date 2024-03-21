fn d2((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32{
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx*dx + dy*dy).sqrt()
}

fn main(){
    let point1 = (4.0, 5.6);
    let point2 = (2.5, 7.8);
    let distance = d2(point1, point2);

    println!("Dystans między punktami {:?} i {:?} wynosi: {}", point1, point2, distance);
}