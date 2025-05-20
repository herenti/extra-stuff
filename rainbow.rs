pub struct Rainbow;

impl Rainbow {
    pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match h {
            h if (0.0..60.0).contains(&h) => (c, x, 0.0),
            h if (60.0..120.0).contains(&h) => (x, c, 0.0),
            h if (120.0..180.0).contains(&h) => (0.0, c, x),
            h if (180.0..240.0).contains(&h) => (0.0, x, c),
            h if (240.0..300.0).contains(&h) => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        let to_rgb = |v: f64| ((v + m) * 255.0).round() as u8;
        (to_rgb(r), to_rgb(g), to_rgb(b))
    }

    pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("{:02x}{:02x}{:02x}", r, g, b)
    }

    pub fn rainbow_text(input: &str, size: &str) -> String {
        let length = input.chars().count() as f64;
        let mut result = String::new();

        for (i, c) in input.chars().enumerate() {
            let hue = (360.0 * i as f64 / length) % 360.0;
            let (r, g, b) = Rainbow::hsl_to_rgb(hue, 1.0, 0.5);
            let color = Rainbow::rgb_to_hex(r, g, b);
            result.push_str(&format!("<f x{}{}=\"0\">{}</f>", size, color, c));
        }

        result
    }
}
