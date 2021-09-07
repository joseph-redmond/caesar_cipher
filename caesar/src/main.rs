fn main() {}
pub fn caesar(cipher_text: String, shift_value: u8) -> String {
    cipher_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first_letter_as_binary = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first_letter_as_binary + (c as u8 + shift_value - first_letter_as_binary) % 26)
                    as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(caesar("".to_string(), 14), "");
    }

    #[test]
    fn caesar_rotate_13() {
        assert_eq!(caesar("rust".to_string(), 13), "ehfg");
    }

    #[test]
    fn ignores_nonalphabetic_characters() {
        assert_eq!(caesar("rust rocks!!!".to_string(), 13), "ehfg ebpxf!!!");
    }

    #[test]
    fn reverses_cipher() {
        assert_eq!(
            caesar("ehfg ebpxf!!!".to_string(), 13),
            "rust rocks!!!".to_string()
        );
    }
}
