use std::str::FromStr;

use num_primes::{ Generator };
use num::BigInt;
use tauri::Window;

use crate::cryp::converter;

#[derive(Clone, serde::Serialize)]
struct Payload {
    waht: String,
    stap: u32,
    from: u32,
}
pub struct Keys {
    pub private: PrivateKey,
    pub public: PublicKey,
}
pub struct PrivateKey {
    pub p: BigInt,
    pub q: BigInt,
    pub d: BigInt,
}
pub struct PublicKey {
    pub e: BigInt,
    pub n: BigInt,
}

pub fn generate_keys(window: tauri::Window) -> Keys {
    send_msg(
        "RSA-Stap",
        Payload {
            waht: "Generate Random Prime number p".into(),
            stap: 0,
            from: 7,
        },
        &window
    );
    let p = BigInt::new(num_bigint::Sign::Plus, Generator::new_prime(2048).to_u32_digits());
    send_msg(
        "RSA-Stap",
        Payload {
            waht: "Generate Random Prime number q".into(),
            stap: 1,
            from: 7,
        },
        &window
    );
    let q = BigInt::new(num_bigint::Sign::Plus, Generator::new_prime(2048).to_u32_digits());
    send_msg(
        "RSA-Stap",
        Payload {
            waht: "Calculate number N from p * q".into(),
            stap: 2,
            from: 7,
        },
        &window
    );
    let n = BigInt::from(p.clone() * q.clone());
    send_msg(
        "RSA-Stap",
        Payload {
            waht: "Calculate number PhiN from (p - 1) * (q - 1)".into(),
            stap: 3,
            from: 7,
        },
        &window
    );
    let phin: BigInt = (p.clone() - 1) * (q.clone() - 1);
    send_msg(
        "RSA-Stap",
        Payload {
            waht: "Calculate number N from p * q".into(),
            stap: 4,
            from: 7,
        },
        &window
    );
    let e = BigInt::new(num_bigint::Sign::Plus, Generator::new_prime(2048).to_u32_digits());
    send_msg(
        "RSA-Stap",
        Payload {
            waht: "Generate Random Prime number e".into(),
            stap: 5,
            from: 7,
        },
        &window
    );
    let mut d = euklid(phin.clone(), e.clone());
    if d.sign() == num_bigint::Sign::Minus {
        d += &phin;
    }
    send_msg(
        "RSA-Stap",
        Payload {
            waht: "Generate Keys".into(),
            stap: 6,
            from: 7,
        },
        &window
    );
    // println!("p:{p}\nq:{q}\nn:{n}\nphin:{phin}\ne:{e}\nd:{d}");
    let new_keys = Keys {
        public: PublicKey { e, n },
        private: PrivateKey { p, q, d },
    };
    return new_keys;
}
pub fn euklid(mut a: BigInt, mut b: BigInt) -> BigInt {
    let mut s = BigInt::new(num_bigint::Sign::Plus, vec![1]);
    let mut t = BigInt::new(num_bigint::Sign::Plus, vec![0]);
    let mut u = BigInt::new(num_bigint::Sign::Plus, vec![0]);
    let mut v = BigInt::new(num_bigint::Sign::Plus, vec![1]);
    while b != BigInt::new(num_bigint::Sign::Plus, vec![0]) {
        let q = &a / &b;
        let b1 = b.clone(); // Variable zum Zwischenspeichern
        b = &a - &q * &b;
        a = b1;
        let u1 = u.clone(); // Variable zum Zwischenspeichern
        u = &s - &q * &u;
        s = u1;
        let v1 = v.clone(); // Variable zum Zwischenspeichern
        v = &t - &q * &v;
        t = v1;
    }
    return t;
}

fn send_msg(name: &str, msg: Payload, window: &Window) {
    let _ = window.emit(name, msg.to_owned());
}
pub fn encrypt(n: BigInt, e: BigInt, msg: String) -> Option<BigInt> {
    let bigint_bytes_msg = BigInt::from_bytes_le(num_bigint::Sign::Plus, msg.as_bytes());
    if bigint_bytes_msg > n {
        return None;
    }
    return Option::Some(bigint_bytes_msg.modpow(&e, &n));
}
pub fn dectypt(n: BigInt, d: BigInt, msg: String) -> BigInt {
    let bigint_bytes_msg = converter::base32_to_dec(msg);
    return bigint_bytes_msg.modpow(&d, &n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euklid() {
        let p = BigInt::new(num_bigint::Sign::Plus, vec![11]);
        let q = BigInt::new(num_bigint::Sign::Plus, vec![13]);
        let e = BigInt::new(num_bigint::Sign::Plus, vec![23]);
        let phin = (p - 1) * (q - 1);
        let d = euklid(phin, e);
        assert_eq!(d, BigInt::new(num_bigint::Sign::Plus, vec![47]));
    }
}
