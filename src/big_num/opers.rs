use std::ops;
use std::fmt::Write;
use std::str;
use crate::big_num::UBigNum;
//======================================================================================================
//===========================================BITWISE OPERATIONS=========================================
//======================================================================================================
// INV
impl ops::Not for UBigNum {
    type Output = Self;
    fn not(mut self) -> Self {
        for i in 0..self.num.len() {
            self.num[i] = !self.num[i];
        }
        Self {
            num: self.num
        }
    }
}
//======================================================================================================
// XOR
impl ops::BitXor for UBigNum {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        let mut vec_result: Vec<u64> = vec![];

        let smaller: usize;
        let bigger: usize;

        if self.num.len() < other.num.len() {
            smaller = self.num.len();
            bigger = other.num.len();
        }
        else {
            smaller = other.num.len();
            bigger = self.num.len();
        }

        for i in 0..smaller {
            vec_result.push(self.num[i] ^ other.num[i]);
        }

        if bigger == other.num.len() {
            for i in smaller..bigger {
                vec_result.push(other.num[i]);
            }
        }
        else {
            for i in smaller..bigger {
                vec_result.push(self.num[i]);
            }
        }

        Self {
            num: vec_result
        }
    }
}
//======================================================================================================
// OR
impl ops::BitOr for UBigNum {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        let mut vec_result: Vec<u64> = vec![];

        let smaller: usize;
        let bigger: usize;

        if self.num.len() < other.num.len() {
            smaller = self.num.len();
            bigger = other.num.len();
        }
        else {
            smaller = other.num.len();
            bigger = self.num.len();
        }

        for i in 0..smaller {
            vec_result.push(self.num[i] | other.num[i]);
        }
        
        if bigger == other.num.len() {
            for i in smaller..bigger {
                vec_result.push(other.num[i]);
            }
        }
        else {
            for i in smaller..bigger {
                vec_result.push(self.num[i]);
            }
        }

        Self {
            num: vec_result
        }
    }
}
//======================================================================================================
// AND
impl ops::BitAnd for UBigNum {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        let mut vec_result: Vec<u64> = vec![];

        let smaller: usize;

        if self.num.len() < other.num.len() {
            smaller = self.num.len();
        }
        else {
            smaller = other.num.len();
        }

        for i in 0..smaller {
            vec_result.push(self.num[i] & other.num[i]);
        }

        Self {
            num: vec_result
        }
    }
}
//======================================================================================================
// ShiftR
impl ops::Shr<u128> for UBigNum {
    type Output = Self;
    fn shr(mut self, n: u128) -> Self {
        let mut num_str = String::new();
        for block in self.num.iter().rev() {
            write!(&mut num_str, "{:b}", block).unwrap();
        }

        if num_str.len() <= n as usize {
            return Self {
                num: vec![0u64]
            }
        }

        let num_str = &mut num_str[0..(num_str.len() - n as usize)].to_string();

        self.num = vec![];
        let step = Self::BUFFSIZE * 4;
        for i in (0..num_str.len()).rev().step_by(step) {
            if i >= step {
                self.num.push(u64::from_str_radix(&num_str[(i-step+1)..=i], 2).unwrap());
            }
            else {
                self.num.push(u64::from_str_radix(&num_str[0..=i], 2).unwrap());
            }
        }

        Self {
            num: self.num
        }
    }
}
//======================================================================================================
// ShiftL
impl ops::Shl<u128> for UBigNum {
    type Output = Self;
    fn shl(mut self, n: u128) -> Self {
        let mut num_str = String::new();
        for block in self.num.iter().rev() {
            write!(&mut num_str, "{:b}", block).unwrap();
        }

        let zeros = vec![b'0'; n as usize];
        num_str.push_str(str::from_utf8(&zeros).unwrap());

        self.num = vec![];
        let step = Self::BUFFSIZE * 4;
        for i in (0..num_str.len()).rev().step_by(step) {
            if i >= step {
                self.num.push(u64::from_str_radix(&num_str[(i-step+1)..=i], 2).unwrap());
            }
            else {
                self.num.push(u64::from_str_radix(&num_str[0..=i], 2).unwrap());
            }
        }

        Self {
            num: self.num
        }
    }
}
//======================================================================================================
//=======================================ARITHMETIC OPERATIONS==========================================
//======================================================================================================
// ADD
impl ops::Add for UBigNum {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        let mut carry = 0u8;
        let max: usize;
        if self.num.len() >= other.num.len() {
            max = self.num.len();
        }
        else {
            max = other.num.len();
        }
        for i in 0..max {
            if i == self.num.len() {
                self.num.push(0);
            }
            self.num[i] += carry as u64;
            if i < other.num.len() {
                self.num[i] += other.num[i];
            }
            carry = (self.num[i] >= UBigNum::BASE) as u8;
            if carry != 0 {
                self.num[i] -= UBigNum::BASE;
            }
        }
        Self {
            num: self.num
        }
    }
}
//======================================================================================================
// SUB
impl ops::Sub for UBigNum {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        let mut carry = 0u8;
        let max: usize;
        if self.num.len() >= other.num.len() {
            max = self.num.len();
        }
        else {
            max = other.num.len();
        }
        for i in 0..max {
            let mut singed_num = self.num[i] as i64;
            self.num[i] -= carry as u64;
            if i < other.num.len() {
                singed_num -= other.num[i] as i64;
            }
            carry = (singed_num < 0) as u8;
            if carry != 0 {
                singed_num += UBigNum::BASE as i64;
            }
            self.num[i] = singed_num as u64;
        }
        Self {
            num: self.num
        }
    }
}
//======================================================================================================
impl ops::Mul for UBigNum {
    type Output = Self;
    fn mul(mut self, other: Self) -> Self::Output {
        let mut carry = 0u8;
        if other.num.len() == 1 {
            let other_u64 = other.num[0];
            for i in 0..self.num.len() {
                if i == self.num.len() {
                    self.num.push(0);
                }
                let cur = carry as u64 + self.num[i] * other_u64;
                self.num[i] = cur % Self::BASE;
                carry = (cur / Self::BASE) as u8;
            }
        }
        else { // INCORRECT!!!!!
            let mut result_num = vec![0u64; self.num.len() + other.num.len()];
            for i in 0..self.num.len() {
                for j in 0..other.num.len() {
                    let mut cur = result_num[i+j] as u128 + carry as u128;
                    if j < other.num.len() {
                        cur += self.num[i] as u128 * other.num[j] as u128;
                    }
                    result_num[i+j] = (cur % Self::BASE as u128) as u64;
                    carry = (cur / Self::BASE as u128) as u8;
                }
            }
            self.num = result_num;
        }
        while self.num.len() > 1 && self.num[self.num.len()-1] == 0 {
            self.num.pop();
        }
        Self {
            num: self.num
        }
    }
}
//======================================================================================================