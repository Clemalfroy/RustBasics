pub fn encode(s: &str, shift: u32) -> Option<String> {
    let mut new = String::new();

    if shift > 26 { return None }

    for mut c in s.chars() {
        if !c.is_alphabetic() { return None }
        c.make_ascii_lowercase();
        let mut value = c as u32 + shift;
        if value > 'z' as u32 {
            value -= 26
        }
        if let Some(x) = std::char::from_u32(value) {
            new.push(x);
        }
   }
   Some(new)
}

pub fn decode(s: &str, shift: u32) -> Option<String> {
    let mut new = String::new();

    if shift > 26 { return None }

    for mut c in s.chars() {
        if !c.is_alphabetic() { return None }
        c.make_ascii_lowercase();
        let mut value = c as u32 - shift;
        if value < 'a' as u32 {
            value += 26
        }
        if let Some(x) = std::char::from_u32(value) {
            new.push(x);
        }
   }
   Some(new)
}

#[cfg(test)]
mod tests {

    use {encode, decode};

    #[test]
    fn test_encode() {
        assert_eq!(encode("hello", 1),  Some(String::from("ifmmp")));
        assert_eq!(encode("halloz", 16),  Some(String::from("xqbbep")));
        assert_eq!(encode("az", 26),  Some(String::from("az")));
        assert_eq!(encode("az", 27),  None);
        assert_eq!(encode("az1", 1),  None);
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("ifmmp", 1),  Some(String::from("hello")));
        assert_eq!(decode("xqbbep", 16),  Some(String::from("halloz")));
        assert_eq!(decode("az", 26),  Some(String::from("az")));
        assert_eq!(decode("az", 27),  None);
        assert_eq!(decode("az1", 1),  None);
    }

}