#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
    pub fn multiply_by(&self, rhs: f64) -> Self {
        let mut color = RGB::new(0, 0, 0);

        if (self.red as f64 * rhs) > 255.0 {
            color.red = 255;
        } else {
            color.red = (self.red as f64 * rhs) as u8;
        }
        if (self.green as f64 * rhs) > 255.0 {
            color.green = 255;
        } else {
            color.green = (self.green as f64 * rhs) as u8;
        }
        if (self.blue as f64 * rhs) > 255.0 {
            color.blue = 255;
        } else {
            color.blue = (self.blue as f64 * rhs) as u8;
        }
        return color;
    }
}
