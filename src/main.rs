/*
Task was acomplished for Distributed Lab's Academy.
This project is long arithmetic realization for unsigned integer. According to task, next operations was implemented:
1.  set_hex() - setting a hex value to humber
    get_hex() - getting a hex value of number via string
2.  Bitwise operations:
    INV
    XOR
    OR
    AND
    shiftR
    shiftL
3. Arithmetic operations:
    ADD
    SUB
    MUL - works only big int and small int mulitply. Two int multiply works incorrect 
Operaitons (2) and (3) was realized via native operation overloading.
Operations that have not yet been implemented:
    MOD
    DIV
    POWMOD
Because the structure that implements long arithmetic uses a vector as the main field, so there is
a change of ownership. Because of this, the lines in which the correct response is checked are commented. 
                    
                    
                    Project was written with Rust lang.
                    Project was written by Victor Katsiuba (victorkatsiuba2k23@gmail.com)
*/


// #![allow(dead_code)]
#![allow(non_snake_case)]
// #![allow(unused_assignments)]
// #![allow(unused_attributes)]
// #![allow(unused_imports)]
// #![allow(unused_macros)]
// #![allow(unused_mut)]
// #![allow(unused_variables)]

mod big_num;

use big_num::UBigNum;

fn main() {
    let mut A = UBigNum::new();
    // 1. set_hex and get_hex testing
    A.set_hex("aabbccddeeff0011");
    assert_eq!("aabbccddeeff0011", A.get_hex());
    A.set_hex("e035c6cfa42609b998b883bc1699df885cef74e2b2cc372eb8fa7e7");
    assert_eq!("e035c6cfa42609b998b883bc1699df885cef74e2b2cc372eb8fa7e7", A.get_hex());
    
    // 2. Bitwise operations testing
    // 2.1 NOT
    A.set_hex("aabbccddee");
    A = !A;
    assert_eq!("ffffff5544332211", A.get_hex());
    A = !A;
    assert_eq!("aabbccddee", A.get_hex());
    A.set_hex("e035c6cfa42609b998b883bc1699df885cef74e2b2cc372eb8fa7e7");
    A = !A;
    assert_eq!("fff1fca39305bd9fff64667477c43e96ff62077a3108b1d4ffd33c8d14705818", A.get_hex());
    A = !A;
    assert_eq!("e035c6cfa42609b998b883bc1699df885cef74e2b2cc372eb8fa7e7", A.get_hex());
    
    // 2.2 XOR
    A.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4");
    let mut B = UBigNum::new();
    B.set_hex("403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c");
    //assert_eq!("1182d8299c0ec40ca8bf3f49362e95e4ecedaf82bfd167988972412095b13db8", (A ^ B).get_hex());
    A.set_hex("6b574a943f85e7b6");
    B.set_hex("12a4b");
    //assert_eq!("6b574a943f84cdfd", (A ^ B).get_hex());
    A.set_hex("12a4b");
    B.set_hex("6b574a943f85e7b6");
    //assert_eq!("6b574a943f84cdfd", (A ^ B).get_hex());
    
    // 2.3 OR
    A.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4");
    let mut B = UBigNum::new();
    B.set_hex("403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c");
    //assert_eq!("51bff8ad9cafd72eabffbfc9befffffffcffbffaffdd779afdf3d7fdf7f73fbc", (A | B).get_hex());
    A.set_hex("6b574a943f85e7b6");
    let mut B = UBigNum::new();
    B.set_hex("12a4b");
    //assert_eq!("6b574a943f85efff", (A | B).get_hex());
    A.set_hex("12a4b");
    B.set_hex("6b574a943f85e7b6");
    //assert_eq!("6b574a943f85efff", (A | B).get_hex());

    // 2.4 AND
    A.set_hex("51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4");
    let mut B = UBigNum::new();
    B.set_hex("403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c");
    //assert_eq!("403d208400a11322034808088d16a1b10121078400c1002748196dd62460204", (A & B).get_hex());
    A.set_hex("6b574a943f85e7b6");
    let mut B = UBigNum::new();
    B.set_hex("12a4b");
    //assert_eq!("12202", (A & B).get_hex());
    A.set_hex("12a4b");
    B.set_hex("6b574a943f85e7b6");
    //assert_eq!("12202", (A & B).get_hex());

    // 2.5 ShiftR
    A.set_hex("1f2d3d4c5b6a");
    //assert_eq!("3e5c7a98b6d", (A >> 3).get_hex());
    A.set_hex("1f2d3d4c5b6a");
    //assert_eq!("7cb4f5316", (A >> 10).get_hex());
    A.set_hex("1f2d3d4c5b6a");
    //assert_eq!("0", (A >> 50).get_hex());

    // 2.6 ShiftR
    A.set_hex("1f2d3d4c5b6a");
    //assert_eq!("f969ea62db50", (A << 3).get_hex());
    A.set_hex("1f2d3d4c5b6a");
    //assert_eq!("7cb4f5316da800", (A << 10).get_hex());
    A.set_hex("1f2d3d4c5b6a");
    //assert_eq!("7cb4f5316da8000000000000", (A << 50).get_hex());

    // 3. Arithmetic operations testing
    // 3.1 ADD
    A.set_hex("36f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80");
    B.set_hex("70983d692f648185febe6d6fa607630ae68649f7e6fc45b94680096c06e4fadb");
    //assert_eq!("a78865c13b14ae4e25e90771b54963ee2d68c0a64d4a8ba7c6f45ee0e9daa65b", (A + B).get_hex());
    A.set_hex("abcdef012345");
    B.set_hex("9876");
    //assert_eq!("abcdef01bbbb", (A + B).get_hex());
    A.set_hex("9876");
    B.set_hex("abcdef012345");
    //assert_eq!("abcdef01bbbb", (A + B).get_hex());

    // 3.2 SUB
    A.set_hex("33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc");
    B.set_hex("22e962951cb6cd2ce279ab0e2095825c141d48ef3ca9dabf253e38760b57fe03");
    //assert_eq!("10e570324e6ffdbc6b9c813dec968d9bad134bc0dbb061530934f4e59c2700b9", (A - B).get_hex());
    A.set_hex("abcdef012345");
    B.set_hex("9876");
    //assert_eq!("abcdef008acf", (A - B).get_hex());

    // 3.3 MUL
    A.set_hex("abcdef012345");
    B.set_hex("9876");
    assert_eq!("665176d4d77739ce", (A * B).get_hex());
}