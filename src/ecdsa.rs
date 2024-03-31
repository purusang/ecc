use ec_generic::{EllipticCurve, FiniteField, Point};
use num_bigint::{BigInt, BigUint, RandBigInt};
use sha256::digest;
struct ECDSA {
    ec: EllipticCurve,
    a_gen: Point,
    q_order: BigUint, //order of the group
}

impl ECDSA {
    // Generates
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        let priv_key = self.generate_priv_key();
        let pub_key = self.generate_pub_key(&priv_key);

        (priv_key, pub_key)
    }
    pub fn generate_priv_key(&self) -> BigUint {
        self.generate_random_positive_no_less_than(&self.q_order)
    }
    // (0, q)
    pub fn generate_random_positive_no_less_than(&self, q: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), q)
    }
    pub fn generate_pub_key(&self, priv_key: &BigUint) -> Point {
        self.ec
            .scalar_mul(&self.a_gen, priv_key)
            .expect("Could not generate Pub Key.")
    }
    // returns (r,s)
    pub fn sign(&self, priv_key: &BigUint, hash: &BigUint) -> (BigUint, BigUint) {
        // R = kA
        // r = x-component( R )
        // s = ( hash(msg) + d*r ) k^-1
        let k = self.generate_random_positive_no_less_than(&self.q_order);
        let R = EllipticCurve::scalar_mul(&self.ec, &self.a_gen, &k).expect("Could not gen R");

        if let Point::Coor(r, _) = R {
            let dr = FiniteField::mult(&priv_key, &r, &self.q_order).expect("Could not d*rmod p");
            let hash_plus_dr = FiniteField::add(&hash, &dr, &self.q_order).expect("could not add");
            let k_inv =
                FiniteField::inv_mult_prime(&k, &self.q_order).expect("Could not inverse k");
            let s =
                FiniteField::mult(&hash_plus_dr, &k_inv, &self.q_order).expect("Could not find s");
            return (r, s);
        }
        panic!("Error while generating signature");
    }
    //// u1 = s^-1 * hash(msg) mod q
    //// u2 = s^-1 * r mod q
    //// P = u1 A + u2 B mod q = (xp, yp)       # A is generator and B is pub key.
    //// if r == xp return 1
    pub fn verify(&self, hash: &BigUint, signature: &(BigUint, BigUint), pub_key: &Point) -> bool {
        let (r, s) = signature;

        let s_inv =
            FiniteField::inv_mult_prime(&s, &self.q_order).expect("Could not get s inverse");
        let u1 = FiniteField::mult(&s_inv, hash, &self.q_order)
            .expect("Could not multiply hash and s inv");
        let u2 = FiniteField::mult(&s_inv, &r, &self.q_order).expect("Could not compute u2");
        let u1a = EllipticCurve::scalar_mul(&self.ec, &self.a_gen, &u1).expect("Error in u1 A");
        let u1b = EllipticCurve::scalar_mul(&self.ec, &pub_key, &u2).expect("Error in u1 A");
        let p = EllipticCurve::add(&self.ec, &u1a, &u1b).expect("Could not compute point P");

        if let Point::Coor(xp, _) = p {
            if xp == *r {
                return true;
            }
        }
        return false;
    }
    pub fn generate_hash_less_than(&self, message: &str, max: &BigUint) -> BigUint {
        let hash = digest(message);
        let hash_bytes = hex::decode(hash).expect("Could not decode hash");
        let hash = BigUint::from_bytes_be(&hash_bytes);
        let hash = hash.modpow(&BigUint::from(1u32), &(max - BigUint::from(1u32)));
        hash
    }
}

#[cfg(test)]
mod test {
    use ec_generic::elliptic_curve;

    use super::*;

    #[test]
    fn test_sign_verify() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        let ecdsa = ECDSA {
            ec: elliptic_curve,
            a_gen: Point::Coor(BigUint::from(5u32), BigUint::from(1u32)),
            q_order: BigUint::from(19u32),
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_pub_key(&priv_key);

        let hash = ecdsa.generate_hash_less_than("Hello World!", &ecdsa.q_order);
        let signature = ecdsa.sign(&priv_key, &hash);
        println!("Signature: {:?}", signature);

        println!("Verify {}", ecdsa.verify(&hash, &signature, &pub_key));
        assert!(ecdsa.verify(&hash, &signature, &pub_key));
    }
}
