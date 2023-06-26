use num::{ BigInt, Num };

pub fn dec_to_hex(dec: BigInt) -> String {
    dec.to_str_radix(16)
}
pub fn dec_to_base32(dec: BigInt) -> String {
    dec.to_str_radix(32)
}
pub fn hex_to_dec(hex: String) -> BigInt {
    BigInt::from_str_radix(&hex, 16).unwrap()
}
pub fn base32_to_dec(base32: String) -> BigInt {
    BigInt::from_str_radix(&base32, 32).unwrap()
}
