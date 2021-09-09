pub struct Caesar {
    
}

impl Caesar {
    pub fn encode(message: String, key: u8) -> String {
        message.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_uppercase() {
                    (((c as u8) - ('A' as u8) + key) % 26 + ('A' as u8)) as char
                } else {
                    (((c as u8) - ('a' as u8) + key) % 26 + ('a' as u8)) as char
                }
            } else {
                c
            }
        }).collect()
    }

    pub fn decode(ciphtext: String, key: u8) -> String {
        ciphtext.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_uppercase() {
                    (((c as u8) - ('A' as u8) - key) % 26 + ('A' as u8)) as char
                } else {
                    (((c as u8) - ('a' as u8) - key) % 26 + ('a' as u8)) as char
                }
            } else {
                c
            }
        }).collect()
    }
}