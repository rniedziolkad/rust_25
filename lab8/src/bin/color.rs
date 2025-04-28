#[derive(PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn from_3u8(r: u8, g: u8, b: u8) -> Rgb {
        Rgb{r, g, b}
    }

    fn from_3percent(rp: f32, gp: f32, bp: f32) -> Option<Rgb> {
        if rp < 0.0 || gp < 0.0 || bp < 0.0 ||
            rp > 100.0 || gp > 100.0 || bp > 100.0 {
                return None;
            }
        
        Some(Rgb { 
            r: (255.0*(rp/100.0)) as u8, 
            g: (255.0*(gp/100.0)) as u8, 
            b: (255.0*(bp/100.0)) as u8 })
    }

    fn gray(p: f32) -> Option<Rgb> {
        Self::from_3percent(p, p, p)
    }

    fn white() -> Rgb {
        Rgb{r: 255, g: 255, b: 255}
    }

    fn black() -> Rgb {
        Rgb{r: 0, g: 0, b: 0}
    }

    fn invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }

    fn intensity(&self) -> f32 {
        (self.r as f32 + self.g as f32 + self.b as f32) / (3.0*255.0)
    }

    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    fn as_cmy_u8tuple(&self) -> (u8, u8, u8) {
        (255-self.r, 255-self.g, 255-self.b)
    }

}

fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0/3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));
}
