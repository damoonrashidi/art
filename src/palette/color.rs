use std::fmt::Display;

/// Color representation
#[derive(Clone, Copy, Debug)]
pub enum Color {
    /**
    HSLa representation of a color. Good for when you want to do small variations to a given color.

    Example:

    ```
    use art::palette::color::Color::HSLa;
    let bright_red = HSLa(0, 50.0, 65.0, 1.0);
    ```
    */
    HSLa(u16, f64, f64, f64),

    /**
    Hex representation for a color.

    Example:

    ```
    use art::palette::color::Color::Hex;
    let bright_red = Hex("#f00");
    ```
    */
    Hex(&'static str),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(color) => write!(f, "{color}"),
            Color::HSLa(h, s, l, a) => {
                write!(f, "hsla({h}, {s:.1}%, {l:.1}%, {a:.2})")
            }
        }
    }
}

impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        match self {
            Color::Hex(color) => {
                let color = color.trim_start_matches('#');
                let r = u8::from_str_radix(&color[0..2], 16).unwrap();
                let g = u8::from_str_radix(&color[2..4], 16).unwrap();
                let b = u8::from_str_radix(&color[4..6], 16).unwrap();

                image::Rgb([r, g, b])
            }
            Color::HSLa(h, s, l, _) => {
                let h = h as f64 / 360.0;
                let s = s / 100.0;
                let l = l / 100.0;

                let q = if l < 0.5 {
                    l * (1.0 + s)
                } else {
                    l + s - l * s
                };
                let p = 2.0 * l - q;

                let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
                let g = hue_to_rgb(p, q, h);
                let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

                image::Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8])
            }
        }
    }
}

fn hue_to_rgb(p: f64, q: f64, h: f64) -> f64 {
    let h = if h < 0.0 {
        h + 1.0
    } else if h > 1.0 {
        h - 1.0
    } else {
        h
    };

    if h * 6.0 < 1.0 {
        p + (q - p) * 6.0 * h
    } else if h * 2.0 < 1.0 {
        q
    } else if h * 3.0 < 2.0 {
        p + (q - p) * (2.0 / 3.0 - h) * 6.0
    } else {
        p
    }
}

#[cfg(test)]
mod test {
    use image::Rgb;

    use super::Color;

    #[test]
    fn render_hsla() {
        let color = Color::HSLa(360, 0.0, 0.0, 1.0);

        assert_eq!("hsla(360, 0.0%, 0.0%, 1.00)", color.to_string());
    }

    #[test]
    fn render_hex() {
        let color = Color::Hex("#111");
        assert_eq!("#111", color.to_string());
    }

    #[test]
    fn into_rgb() {
        let rgb: Rgb<u8> = Color::Hex("#ff0000").into();
        assert_eq!(rgb, image::Rgb([255, 0, 0]));
    }
}
