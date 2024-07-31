/// A trait for padding a string to a specified width.
pub trait PadToWidth {
    /// Pads the string to the specified width with the specified padding character.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired width of the padded string.
    /// * `pad_char` - The character used for padding.
    ///
    /// # Returns
    ///
    /// The padded string.
    fn pad_to_width(&self, width: usize, pad_char: char) -> String;
}

impl PadToWidth for String {
    fn pad_to_width(&self, width: usize, pad_char: char) -> String {
        let mut string = self.clone();
        while string.len() < width {
            string.insert(0, pad_char);
        }
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_to_width() {
        let string = String::from("Hello");
        assert_eq!(string.pad_to_width(10, '-'), "-----Hello");

        let string = String::from("World");
        assert_eq!(string.pad_to_width(5, '*'), "World");

        let string = String::from("Rust");
        assert_eq!(string.pad_to_width(7, '#'), "###Rust");

        let string = String::from("Programming");
        assert_eq!(string.pad_to_width(15, '+'), "++++Programming");
    }
}
