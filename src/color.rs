#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Rgb {
    /// Individually multiplies the `red`, `green` and `blue` components by
    /// the value passed, and returns a new `Rgb` instance with the
    /// result.
    pub fn multiply_by(&self, m: f64) -> Rgb {
        Self {
            red: self.red * m,
            green: self.green * m,
            blue: self.blue * m,
        }
    }

    /// Individually adds the `red`, `green` and `blue` components with the
    /// corresponding components of the `Rgb` instance passed, and
    /// returns a new `Rgb` instance with the result.
    pub fn add(&self, a: &Rgb) -> Rgb {
        Self {
            red: self.red + a.red,
            green: self.green + a.green,
            blue: self.blue + a.blue,
        }
    }

    /// Individually clamps the `red`, `green` and `blue` components to the range
    /// 0-255. Negative values become 0 and values greater than 255 become
    /// 255.
        #[rustfmt::skip]
    pub fn clamp(&self) -> Rgb {
            Rgb {
                red:   f64::min(255.0, f64::max(0.0, self.red)),
                green: f64::min(255.0, f64::max(0.0, self.green)),
                blue:  f64::min(255.0, f64::max(0.0, self.blue)),
            }
        }

    /// Creates a new `Rgb` instance from the `red`, `green` and `blue`
    /// values passed.
    pub fn from_ints(red: i16, green: i16, blue: i16) -> Rgb {
        Rgb {
            red: red as f64,
            green: green as f64,
            blue: blue as f64,
        }
    }
}
