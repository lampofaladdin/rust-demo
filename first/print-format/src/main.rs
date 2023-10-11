use std::fmt::{self, write, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}:{:.3}°{},{:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({},{},{}) 0x{:02X?}{:02X?}{:02X?}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.123123,
            lon: -123.312231,
        },
        City {
            name: "Oslo",
            lat: -53.123123,
            lon: 123.31221,
        },
        City {
            name: "Vancouver",
            lat: -53.123123,
            lon: -123.312231,
        },
    ]
    .iter()
    {
        println!("{}", city);
    }

    for color in [
        Color {
            red: 255,
            green: 0,
            blue: 128,
        },
        Color {
            red: 0,
            green: 255,
            blue: 128,
        },
        Color {
            red: 128,
            green: 0,
            blue: 255,
        },
    ]
    .iter()
    {
        println!("{}", color);
    }
}
