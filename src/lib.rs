use core::num;

use num_bigint::BigUint;
#[derive(PartialEq, Debug, Clone)]
enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}
struct EllipticCurve {
    // y2 = x3 + ax + b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}
impl EllipticCurve {
    fn add(self: &Self, c: &Point, d: &Point) -> Point {
        assert!(c != d, "Points must be different");
        assert!(self.is_on_curve(&c), "Point is not on curve");
        assert!(self.is_on_curve(&d), "Point is not on curve");
        // s= (y2-y1)/(x2-x1)
        // x3 = s^2 -x1 -x2 mod p
        // y3 = s(x1-x3)-y1 mod p
        match (c, d) {
            (Point::Identity, _) => d.clone(),
            (_, Point::Identity) => c.clone(),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                if x1 == x2 && FiniteField::add(&y1, &y2, &self.p) == BigUint::from(0u32) {
                    return Point::Identity;
                }
                let y2minusy1 = FiniteField::subtract(&y2, &y1, &self.p);
                let x2minusx1 = FiniteField::subtract(&x2, &x1, &self.p);
                let s = FiniteField::divide(&y2minusy1, &x2minusx1, &self.p);
                let s2 = s.modpow(&BigUint::from(2u32), &self.p);
                let s2minusx1 = FiniteField::subtract(&s2, &x1, &self.p);
                let x3 = FiniteField::subtract(&s2minusx1, &x2, &self.p);

                let x1minusx3 = FiniteField::subtract(&x1, &x3, &self.p);
                let sx1minusx3 = FiniteField::mult(&s, &x1minusx3, &self.p);
                let y3 = FiniteField::subtract(&sx1minusx3, &y1, &self.p);
                Point::Coordinate(x3, y3)
            }
        }
    }
    fn doubling(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(&c), "Point is not on curve");
        // s= (3 * x1^2 + a) / (2 * y1 ) mod p
        // x3 = s^2 - 2 *x1 mod p
        // y3 = s (x1 - x3) - y1 mod p
        match c {
            Point::Identity => Point::Identity,
            Point::Coordinate(x1, y1) => {
                let numerator = x1.modpow(&BigUint::from(2u32), &self.p);
                let numerator = FiniteField::mult(&BigUint::from(3u32), &numerator, &self.p);
                let numerator = FiniteField::add(&numerator, &self.a, &self.p);

                let denominator = FiniteField::mult(&BigUint::from(2u32), &y1, &self.p);
                let s = FiniteField::divide(&numerator, &denominator, &self.p);
                let x2 = &x1;

                self.compute_x3_y3(&s, x1, y1, x2)
            }
        }
    }
    fn compute_x3_y3(&self, s: &BigUint, x1: &BigUint, y1: &BigUint, x2: &BigUint) -> Point {
        let s2 = s.modpow(&BigUint::from(2u32), &self.p);
        let s2minusx1 = FiniteField::subtract(&s2, &x1, &self.p);
        let x3 = FiniteField::subtract(&s2minusx1, &x2, &self.p);

        let x1minusx3 = FiniteField::subtract(&x1, &x3, &self.p);
        let sx1minusx3 = FiniteField::mult(&s, &x1minusx3, &self.p);
        let y3 = FiniteField::subtract(&sx1minusx3, &y1, &self.p);
        Point::Coordinate(x3, y3)
    }
    fn is_on_curve(self: &Self, c: &Point) -> bool {
        match c {
            Point::Coordinate(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mult(&self.a, &x, &self.p);
                let x3plusax = FiniteField::add(&x3, &ax, &self.p);
                let x2plusaxplusb = FiniteField::add(&x3plusax, &self.b, &self.p);
                y2 == x2plusaxplusb
            }
            Point::Identity => true,
        }
    }
}
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
    fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_neg = Self::inv_addition(d, p);
        Self::add(c, &d_neg, p)
    }
    fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv = Self::inv_multiplication(d, p);
        Self::mult(c, &d_inv, p)
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

    #[test]
    fn test_point_in_curve() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let p3 = Point::Coordinate(BigUint::from(10u32), BigUint::from(6u32));

        assert!(ec.is_on_curve(&p1));
        assert!(ec.is_on_curve(&p2));
        assert!(ec.is_on_curve(&p3));

        let p4 = Point::Coordinate(BigUint::from(4u32), BigUint::from(1u32));
        let p5 = Point::Coordinate(BigUint::from(1u32), BigUint::from(1u32));
        let p6 = Point::Coordinate(BigUint::from(0u32), BigUint::from(1u32));

        assert!(!ec.is_on_curve(&p4));
        assert!(!ec.is_on_curve(&p5));
        assert!(!ec.is_on_curve(&p6));
    }
    #[test]
    fn test_point_addition() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coordinate(BigUint::from(10u32), BigUint::from(6u32));

        let res = ec.add(&p1, &p2);
        assert_eq!(res, pr);

        let res = ec.add(&p2, &p1);
        assert_eq!(res, pr);
    }
    #[test]
    fn test_point_addition_reflection() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5,16) + (5,1) = Identity
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(16u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Identity;

        let res = ec.add(&p1, &p2);
        assert_eq!(res, pr);

        let res = ec.add(&p2, &p1);
        assert_eq!(res, pr);
    }
    #[test]
    fn test_doubling() {
        // y^2 = x^3 + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5,1) + (5,1) = 2* (5,1) = (6, 3)
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        // let pr = Point::Identity;

        let res = ec.doubling(&p1);
        assert_eq!(res, pr);
    }
}
