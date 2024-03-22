use num_bigint::BigUint;

struct FiniteField {}
impl FiniteField {
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        (c + d).modpow(&BigUint::from(1u32), &p)
    }
    fn mult(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        (c * d).modpow(&BigUint::from(1u32), &p)
    }
    fn inv_addition(c: &BigUint, p: &BigUint) -> BigUint {
        assert!(c < p, "c is greater than p");
        p - c
    }
    fn inv_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        (c).modpow(&(p - BigUint::from(2u32)), p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let c = BigUint::from(2u32);
        let d = BigUint::from(2u32);
        let p = BigUint::from(10u32);
        assert_eq!(FiniteField::add(&c, &d, &p), BigUint::from(4u32));
    }
    #[test]
    fn add1() {
        let c = BigUint::from(2u32);
        let d = BigUint::from(2u32);
        let p = BigUint::from(3u32);
        assert_eq!(FiniteField::add(&c, &d, &p), BigUint::from(1u32));
    }
    #[test]
    fn mult() {
        let c = BigUint::from(2u32);
        let d = BigUint::from(3u32);
        let p = BigUint::from(4u32);
        assert_eq!(FiniteField::mult(&c, &d, &p), BigUint::from(2u32));
    }
    #[test]
    #[should_panic]
    fn mult1() {
        let c = BigUint::from(2u32);
        let d = BigUint::from(4u32);
        let p = BigUint::from(3u32);
        assert_eq!(FiniteField::add(&c, &d, &p), BigUint::from(1u32));
    }
    #[test]
    fn inv_addition() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(7u32);
        assert_eq!(FiniteField::inv_addition(&c, &p), BigUint::from(3u32));
    }
    #[test]
    fn inv_multiplication() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(7u32);
        assert_eq!(FiniteField::inv_multiplication(&c, &p), BigUint::from(2u32));
    }
    #[test]
    fn identity_addition() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(7u32);
        let d = FiniteField::inv_addition(&c, &p);
        assert_eq!(FiniteField::add(&c, &d, &p), BigUint::from(0u32));
    }
    #[test]
    fn identity_multiplication() {
        let c = BigUint::from(4u32);
        let p = BigUint::from(7u32);
        let d = FiniteField::inv_multiplication(&c, &p);
        assert_eq!(FiniteField::mult(&c, &d, &p), BigUint::from(1u32));
    }
}
