use bigint::{BigDigit, BigInt, DoubleBigDigit};
use bigint::Sign::*;

use bigint::digit::to_lo_hi;

use std::ops::Mul;

impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> Self::Output { naive_mul(&self, &rhs) }
}

impl<'a, 'b> Mul<&'a BigInt> for &'b BigInt {
    type Output = BigInt;
    fn mul(self, rhs: &'a BigInt) -> Self::Output { naive_mul(self, rhs) }
}

impl Mul<BigDigit> for BigInt {
    type Output = BigInt;
    fn mul(mut self, rhs: BigDigit) -> Self::Output {
        if self.is_zero() || rhs == 0 {
            return BigInt::zero();
        }

        let carry = dmul(&mut self.digits, rhs);

        if carry != 0 {
            self.digits.push(carry);
        }

        self
    }
}


pub(crate) fn naive_mul(lhs: &BigInt, rhs: &BigInt) -> BigInt {
    let sign = lhs.sign * rhs.sign;
    if sign == Zero {
        return BigInt::zero();
    }

    let mut digits = vec![0; lhs.digits.len() + rhs.digits.len()];

    n_mul3(&mut digits, &lhs.digits, &rhs.digits);

    let out = BigInt { sign, digits };
    out.trimmed()
}

/// 3 argument naive multiplication: target += lhs * rhs
pub(crate) fn n_mul3(target: &mut [BigDigit], lhs: &[BigDigit], rhs: &[BigDigit]) {
    assert!(target.len() >= lhs.len() + rhs.len());

    let mut carry: BigDigit = 0;

    for (i, l) in lhs.iter().cloned().enumerate() {
        if l == 0 {
            continue;
        }
        for (j, r) in rhs.iter().cloned().enumerate() {
            let [lo, hi] = to_lo_hi(
                l as DoubleBigDigit * r as DoubleBigDigit + target[i + j] as DoubleBigDigit +
                    carry as DoubleBigDigit,
            );
            target[i + j] = lo;
            if j + 1 != rhs.len() {
                carry = hi;
            } else {
                carry = 0;
                target[i + j + 1] = hi;
            }
        }
    }
}

/// Multiplies a slice by a single BigDigit, returning the carry.
pub(crate) fn dmul(lhs: &mut [BigDigit], rhs: BigDigit) -> BigDigit {
    let rhs = rhs as DoubleBigDigit;
    let mut carry: BigDigit = 0;
    for d in lhs.iter_mut() {
        let [lo, hi] = to_lo_hi((*d as DoubleBigDigit * rhs) + carry as DoubleBigDigit);
        *d = lo;
        carry = hi;
    }
    carry
}


#[cfg(all(target_pointer_width = "64", not(feature = "thicc_ints")))]
#[test]
fn scalar_mul_test_1() {
    use bigint::sign::Sign;

    let y: u32 = 915327;

    let a = BigInt {
        sign: Sign::Positive,
        digits: vec![3059078384, 2360247638, 2634550291, 6],
    };
    let b = BigInt {
        sign: Sign::Positive,
        digits: vec![356004624, 4070707789, 1201864523, 6053427],
    };

    assert_eq!(a * y, b);
}
