pub trait Pad {
    fn pad_left(&self, length: usize, padding_character: char) -> String;
    fn pad_right(&self, length: usize, padding_character: char) -> String;
}

impl<T: AsRef<str>> Pad for T {
    fn pad_left(&self, length: usize, padding_character: char) -> String {
        let padding_length = get_padding_length(self.as_ref(), length);
        let padding = get_padding(padding_character, padding_length);
        
        format!("{}{}", padding, self.as_ref())
    }
    
    fn pad_right(&self, length: usize, padding_character: char) -> String {
        let padding_length = get_padding_length(self.as_ref(), length);
        let padding = get_padding(padding_character, padding_length);
        
        format!("{}{}", self.as_ref(), padding)
    }
}

fn get_padding_length(content: &str, length: usize) -> usize {
    let content_length = content.chars().count();
    if content_length > length {
        0
    } else {
        length - content_length
    }
}

fn get_padding(padding_character: char, length: usize) -> String {
    String::from_utf8(vec![padding_character as u8; length]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::Pad;
    
    #[test]
    fn pad_left_works() {
        assert_eq!("    1", "1".pad_left(5, ' '));
    }
    
    #[test]
    fn pad_right_works() {
        assert_eq!("1    ", "1".pad_right(5, ' '));
    }
}
