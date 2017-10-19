
// Encoding type
#[derive(Debug)]
enum Encoding {
    Bin,
    Oct,
    Dec,
    Hex,
}

// Uint type for less lossy encoding/decoding
#[derive(Debug)]
pub struct Uint {
    pub value:  u32,
    width:      usize,
    encoding:   Encoding,
}

// Equality based only on value
impl PartialEq for Uint {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Uint {
    pub fn parse(text: &str) -> Uint {
        if text.starts_with("0x") || text.starts_with("0X") {
            Uint {
                value: u32::from_str_radix(&text["0x".len()..], 16).unwrap(),
                width: text.len()-2,
                encoding: Encoding::Hex,
            }
        } else if text.starts_with('#') {
            Uint {
                value: u32::from_str_radix(&str::replace(&text["#".len()..], "x", "0"), 2).unwrap(), 
                width: text.len()-1,
                encoding: Encoding::Bin,
            }
        } else if text.starts_with('0') {
            Uint {
                value: u32::from_str_radix(text, 8).unwrap(), 
                width: text.len()-1,
                encoding: Encoding::Oct,
            }
        } else {
            Uint {
                value: text.parse().unwrap(), 
                width: text.len(),
                encoding: Encoding::Dec,
            }
        }
    }

    pub fn encode(&self) -> String {
        match self.encoding {
            Encoding::Dec => {
                let base = &format!("{}", self.value);
                let packing = String::from_utf8(vec!['0' as u8; self.width - base.len()]).unwrap();
                format!("{}{}", packing, base)
            },
            Encoding::Hex => {
                let base = format!("{:.x}", self.value);
                let packing = String::from_utf8(vec!['0' as u8; self.width - base.len()]).unwrap();
                format!("0x{}{}", packing, base)
            },
            Encoding::Oct => {
                let base = &format!("{:o}", self.value);
                let packing = String::from_utf8(vec!['0' as u8; self.width - base.len()]).unwrap();
                format!("0{}{}", packing, base)
            },
            Encoding::Bin => {
                let base = format!("{:b}", self.value);
                let packing = String::from_utf8(vec!['0' as u8; self.width - base.len()]).unwrap();
                format!("#{}{}", packing, base)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uint_decode_encode() {
        let tests = vec![
            ("104", Uint{value: 104, width: 3, encoding: Encoding::Dec}),
            ("0x013a", Uint{value: 314, width: 4, encoding: Encoding::Hex}),
            ("01232", Uint{value: 666, width: 4, encoding: Encoding::Oct}),
            ("#0101", Uint{value: 5, width: 4, encoding: Encoding::Bin}),
        ];

        for (text, value) in tests {
            let a = Uint::parse(text);
            assert_eq!(a, value);
            let b = value.encode();
            assert_eq!(b, text);
        }
    }
}