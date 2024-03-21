fn d2(point1: (f32, f32, f32), point2: (f32, f32, f32)) -> f32{
    let dx = point1.0 - point2.0;
    let dy = point1.1 - point2.1;
    let dz = point1.2 - point2.2;
    (dx*dx + dy*dy + dz * dz).sqrt()
}

fn main(){
    let point1 = (4.0, 5.6, 3.4); // Krokta
    let point2 = (2.5, 7.8, 1.4);
    let distance = d2(point1, point2);

    println!("Dystans między punktami {:?} i {:?} wynosi: {}", point1, point2, distance);
}