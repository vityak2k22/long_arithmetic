pub mod opers;
use std::fmt::Write;

pub struct UBigNum {
    num: Vec<u64>
}

impl UBigNum {
    const BASE: u64 = 10_000_000_000_000_000_000;
    const BUFFSIZE: usize = 14;
//======================================================================================================
    // Create a number with an empty vector initial value
    pub fn new () -> UBigNum {
        return UBigNum {
            num: vec![]
        }
    }
//======================================================================================================
    // Assign a number into hexadecimal notation
    pub fn set_hex (&mut self, num_str: &str) {
        for i in 0..num_str.len() {
            let char = num_str.as_bytes()[i];
            if (char >= b'0' && char <= b'9' ||
                char >= b'A' && char <= b'F' ||
                char >= b'a' && char <= b'f') == false {
                panic!("Hex format is not correct");
            }
        }
        self.num = vec![];
        for i in (0..num_str.len()).rev().step_by(Self::BUFFSIZE) {
            if i >= Self::BUFFSIZE {
                self.num.push(u64::from_str_radix(&num_str[(i-Self::BUFFSIZE+1)..=i], 16).unwrap());
            }
            else {
                self.num.push(u64::from_str_radix(&num_str[0..=i], 16).unwrap());
            }
        }
    }
//======================================================================================================
    // Gettting a hex value of number in string
    pub fn get_hex (&self) -> String {
        let mut num_str = String::new();
        for block in self.num.iter().rev() {
            write!(&mut num_str, "{:x}", block).unwrap();
        }
        num_str
    }
//======================================================================================================
}