use ec_generic::{EllipticCurve, FiniteField, Point};
use num_bigint::BigUint;

struct ECDSA {
    ec: EllipticCurve,
    gen: Point,
    q: BigUint, //order of the group
}

impl ECDSA {
    // Generates
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        let priv_key = self.generate_priv_key();
        let pub_key = self.generate_pub_key();
    }
    pub fn generate_priv_key(&self) -> BigUint {
        generate_random_positive_no_less_than(&self.q)
    }
    pub fn generate_random_positive_no_less_than(q: &BigUint) -> BigUint {
        todo!();
    }
    pub fn generate_pub_key(&self, priv_key: &BigUint) -> Point {
        todo!();
    }
    // returns (r,s)
    pub fn sign(&self, priv_key: &BigUint, hash: &BigUint) -> (BigUint, BigUint) {
        todo!();
    }
    pub fn verify(&self, hash: &BigUint, sign: &(BigUint, BigUint)) -> bool {
        todo!();
    }
}
