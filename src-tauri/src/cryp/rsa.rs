use std::{ ops::Mul };

use num_primes::{ Generator };
use num_bigint::BigInt;

#[derive(Clone, serde::Serialize)]
struct Payload {
    Waht: String,
    Stap: u32,
    From: u32,
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

pub fn GenerateKeys(window: tauri::Window) -> Keys {
    let _ = window.emit("RSA-Stap", Payload { Waht: "Init p".into(), Stap: 1, From: 6 });
    let p = BigInt::new(num_bigint::Sign::Plus, Generator::new_prime(1024).to_u32_digits());
    let _ = window.emit("RSA-Stap", Payload { Waht: "Init q".into(), Stap: 2, From: 6 });
    let q = BigInt::new(num_bigint::Sign::Plus, Generator::new_prime(1024).to_u32_digits());
    let _ = window.emit("RSA-Stap", Payload { Waht: "Init n".into(), Stap: 3, From: 6 });
    let mut n = BigInt::from(p.clone() * q.clone());

    let _ = window.emit("RSA-Stap", Payload { Waht: "Init φn".into(), Stap: 4, From: 6 });
    let phin: BigInt = (p.clone() - 1) * (q.clone() - 1);
    let _ = window.emit("RSA-Stap", Payload { Waht: "Init e".into(), Stap: 5, From: 6 });
    let e = BigInt::new(num_bigint::Sign::Plus, Generator::new_prime(1024).to_u32_digits());
    let _ = window.emit("RSA-Stap", Payload { Waht: "Berechen d".into(), Stap: 6, From: 6 });
    let d = euklid(phin.clone(), e.clone());
    let new_keys = Keys {
        private: PrivateKey { p: p.clone(), q: q.clone(), d: d.clone() },
        public: PublicKey { e: e.clone(), n: n.clone() },
    };
    return new_keys;
}
pub fn euklid(mut a: BigInt, mut b: BigInt) -> BigInt {
    let mut s = BigInt::new(num_bigint::Sign::Plus, vec![0]);
    let mut t = BigInt::new(num_bigint::Sign::Plus, vec![0]);
    let mut u = BigInt::new(num_bigint::Sign::Plus, vec![0]);
    let mut v = BigInt::new(num_bigint::Sign::Plus, vec![1]);
    while b != BigInt::new(num_bigint::Sign::Plus, vec![0]) {
        let q = a.clone() / b.clone();
        let b1 = b.clone();
        b = a.clone() - q.clone() * b.clone();
        a = b1.clone();
        let u1 = u.clone();
        u = s.clone() - q.clone() - u.clone();
        s = u1.clone();
        let v1 = v.clone();
        v = t.clone() - q.clone() * v.clone();
        t = v1.clone();
    }
    return a.to_owned();
}
