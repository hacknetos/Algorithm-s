use num_primes::{ Generator };
use num_bigint::BigInt;

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
    let _ = window.emit("RSA-Stap", Payload {
        waht: "Generate Random Prime number p".into(),
        stap: 0,
        from: 7,
    });
    let p = BigInt::new(
        num_bigint::Sign::Plus,
        vec![101] /*Generator::new_prime(1024).to_u32_digits()*/
    );
    let _ = window.emit("RSA-Stap", Payload {
        waht: "Generate Random Prime number q".into(),
        stap: 1,
        from: 7,
    });
    let q = BigInt::new(
        num_bigint::Sign::Plus,
        vec![103] /*Generator::new_prime(1024).to_u32_digits()*/
    );
    let _ = window.emit("RSA-Stap", Payload {
        waht: "Calculate number N from p * q".into(),
        stap: 2,
        from: 7,
    });
    let mut n = BigInt::from(p.clone() * q.clone());

    let _ = window.emit("RSA-Stap", Payload {
        waht: "Calculate number φN from (p - 1) * (q - 1)".into(),
        stap: 3,
        from: 7,
    });
    let phin: BigInt = (p.clone() - 1) * (q.clone() - 1);
    let _ = window.emit("RSA-Stap", Payload {
        waht: "Generate Random Prime number e".into(),
        stap: 4,
        from: 7,
    });
    let e = BigInt::new(
        num_bigint::Sign::Plus,
        vec![7] /*Generator::new_prime(1024).to_u32_digits()*/
    );
    let _ = window.emit("RSA-Stap", Payload {
        waht: "Calculate number d from e * d = 1 % φN".into(),
        stap: 5,
        from: 7,
    });
    let d = euklid(
        phin.clone(), //phin
        e.clone(), //e
        BigInt::new(num_bigint::Sign::Plus, vec![0]),
        BigInt::new(num_bigint::Sign::Plus, vec![0])
    );
    let _ = window.emit("RSA-Stap", Payload {
        waht: "Create key".into(),
        stap: 6,
        from: 7,
    });
    println!("p:{p}\nq:{q}\nn:{n}\nphin:{phin}\ne:{e}");
    println!("e*d%phin {}", &phin + &d);
    let new_keys = Keys {
        private: PrivateKey { p: p.clone(), q: q.clone(), d: d.clone() },
        public: PublicKey { e: e.clone(), n: n.clone() },
    };
    return new_keys;
}
pub fn euklid(mut a: BigInt, mut b: BigInt, mut s: BigInt, mut t: BigInt) -> BigInt {
    s = BigInt::new(num_bigint::Sign::Plus, vec![1]);
    t = BigInt::new(num_bigint::Sign::Plus, vec![0]);
    let mut u = BigInt::new(num_bigint::Sign::Plus, vec![0]);
    let mut v = BigInt::new(num_bigint::Sign::Plus, vec![1]);
    while b != BigInt::new(num_bigint::Sign::Plus, vec![0]) {
        let mut q = &a / &b;
        let b1 = b.clone(); // Variable zum Zwischenspeichern
        b = &a - &q * &b;
        a = b1;
        let u1 = u.clone(); // Variable zum Zwischenspeichern
        u = &s - &q * &u;
        s = u1;
        let v1 = v.clone(); // Variable zum Zwischenspeichern
        v = &t - &q * &v;
        t = v1;
        println!("{a},{b},{s},{t}");
    }
    println!("s:\n{},\nt:\n{}\n s * t {}", &s, &t, &s * &t);
    return t;
}
