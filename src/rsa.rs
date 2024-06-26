use crate::exponent::fast_modular_exponentiation;
use crate::gcd::gcd;
use crate::inverse::modular_inverse;
use crate::wilson::is_prime2;
use rand::Rng;

#[derive(Debug, Default)]
struct PrivateKey {
    pub n: u64,
    pub d: u64,
}

#[derive(Debug, Default)]
struct PublicKey {
    pub n: u64,
    pub e: u64,
}

impl PrivateKey {
    pub fn decrypt(&self, m: u64) -> u64 {
        fast_modular_exponentiation(m, self.d, self.n)
    }
}

impl PublicKey {
    pub fn encrypt(&self, c: u64) -> u64 {
        fast_modular_exponentiation(c, self.e, self.n)
    }
}

fn gen_key() -> (PrivateKey, PublicKey) {
    let mut rng = rand::thread_rng();

    // generate p,q
    let mut p: u64;
    let mut q: u64;
    loop {
        p = rng.gen::<u32>() as u64;
        if is_prime2(p) {
            break;
        }
    }

    loop {
        q = rng.gen::<u32>() as u64;
        if is_prime2(q) {
            break;
        }
    }

    let n = p * q;
    let phi_n = (p - 1) * (q - 1);

    // generate e
    let mut e;

    loop {
        e = rng.gen::<u64>();
        if e > 1 && e < phi_n && gcd(e as i64, phi_n as i64) == 1 {
            break;
        }
    }

    // calculate d
    let d = modular_inverse(e as i64, phi_n as i64).unwrap() as u64;

    let public_key = PublicKey { e, n };

    let private_key = PrivateKey { n, d };

    (private_key, public_key)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn test_my_rsa() {
        let mut rng = rand::thread_rng();
        let (private, public) = gen_key();
        println!("private key: {:?}", private);
        println!("public key: {:?}", public);
        for _i in 0..1000 {
            let m = rng.gen::<u32>() as u64;
            let c = public.encrypt(m);
            let dm = private.decrypt(c);
            assert_eq!(m, dm);
        }
    }
}
