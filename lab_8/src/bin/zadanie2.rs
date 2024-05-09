#[derive(Debug, PartialEq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8
}

impl Rgb {

    fn from_3u8(r: u8, g: u8, b: u8) -> Rgb{
        Rgb{red: r, green: g, blue: b}
    }

    fn from_3percent(r: f32, g: f32, b: f32) -> Option<Rgb>{

        if r > 100.0 || g > 100.0 || b > 100.0 {
            return None;
        }
        
        let red = (255.0 * (r / 100.0)) as u8;
        let green = (255.0 * (g / 100.0)) as u8;
        let blue = (255.0 * (b / 100.0)) as u8;

        Some(Rgb{red, green, blue})
    }

    fn gray(value: f32) -> Option<Rgb> {

        if value > 100.0 {
            return None;
        }

        let gray = (255.0 * (value / 100.0)) as u8;

        Some(Rgb{red: gray, green: gray, blue: gray})
    }

    fn white() -> Rgb {
        Rgb{red: 255, green: 255, blue: 255}
    }

    fn black() -> Rgb {
        Rgb{red: 0, green: 0, blue: 0}
    }

    fn invert(&mut self) {

        self.red = 255 - self.red;
        self.green = 255 - self.green;
        self.blue = 255 - self.blue;

    }

    fn intensity(&self) -> f32 {
        (self.red + self.green + self.blue) as f32 / (255.0 * 3.0)
    }

    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    fn as_cmy_u8tuple(&self) -> (u8, u8, u8){
        let cyan = 255 - self.red;
        let magenta = 255 - self.green;
        let yellow = 255 - self.blue;

        (cyan, magenta, yellow)
    }

}

fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    println!("{:?}", szary1);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    println!("{:?}", szary2);
    let szary3 = Rgb::gray(50.0).unwrap();
    println!("{:?}", szary3);
    let fiolet = Rgb::from_3u8(100, 35, 120);
    println!("{:?}", fiolet);
    let bialy1 = Rgb::white();
    println!("{:?}", bialy1);
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    println!("{:?}", bialy2);
    let mut czarny1 = Rgb::black();
    println!("{:?}", czarny1);
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    println!("{:?}", czarny2);
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0/3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));

}