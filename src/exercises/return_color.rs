struct Color {
    r: u8,
    g: u8,
    b: u8,
}

enum DefaultColor {
    Red,
    Green,
    Blue,
}

impl Into<Color> for DefaultColor {
    fn into(self) -> Color {
        match self {
            _ => todo!(),
        }
    }
}

fn return_color(input: Option<DefaultColor>) -> Result<Color, ()> {
    match input {
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_return_DefaultColor() {
        let red = return_color(Some(DefaultColor::Red)).unwrap();
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);

        let green = return_color(Some(DefaultColor::Green)).unwrap();
        assert_eq!(green.r, 0);
        assert_eq!(green.g, 255);
        assert_eq!(green.b, 0);

        let blue = return_color(Some(DefaultColor::Blue)).unwrap();
        assert_eq!(blue.r, 0);
        assert_eq!(blue.g, 0);
        assert_eq!(blue.b, 255);

        let none = return_color(None);
        assert!(none.is_err());
    }
}
