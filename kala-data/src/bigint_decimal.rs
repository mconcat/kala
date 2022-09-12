// implementation of decimal arithmetic on uint128

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decimal {
    Number(i128),
    Infinity(bool),
    NaN,
}

const PRECISION: i128 = 10**18;

impl Decimal {
    fn is_positive(&self) -> bool {
        match self {
            Decimal::Number(n) => *n >= 0,
            Decimal::Infinity(n) => !n,
            Decimal::NaN => false,
        }
    }

    fn positive_infinity() -> Self {
        Decimal::Infinity(false)
    }

    fn negative_infinity() -> Self {
        Decimal::Infinity(true)
    }

    fn number(n: i128) -> Self {
        Decimal::Number(n)
    }

    fn handle_exotic(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Decimal::NaN, _) => Decimal::NaN,
            (_, Decimal::NaN) => Decimal::NaN,
            (Decimal::Infinity(n), Decimal::Infinity(m)) => {

            }
        }
    }
 
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::Number(i), Decimal::Number(j)) => {
                let (result, overflow) = i.overflowing_add(j);
                if overflow {
                    if result >= 0 { // underflow
                        Decimal::negative_infinity()
                    } else { // overflow
                        Decimal::positive_infinity()
                    }
                } else {
                    Decimal::number(result)
                }
            }
            _ => self.handle_exotic(rhs)
        }
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::Number(i), Decimal::Number(j)) => {
                let (result, overflow) = self.overflowing_sub(rhs);
                if overflow {
                    if result >= 0 { // underflow
                        Decimal::negative_infinity()
                    } else { // overflow
                        Decimal::positive_infinity()
                    }
                } else {
                    Decimal::number(result)
                }
            }
        }
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::Number(i), Decimal::Number(j)) => {
                let left = self.0.to_le_bytes();
                let right = rhs.0.to_le_bytes();
            },
            (Decimal::Number(0), Decimal::Infinity(_)) => Decimal::NaN,
            (Decimal::Infinity(_), Decimal::Number(0)) => Decimal::NaN,
            (Decimal::Infinity(n), Decimal::Infinity(m)) => {
                if n == m {
                    Decimal::positive_infinity()
                } else {
                    Decimal::negative_infinity()
                }
            }
        }
    }
}

impl Div for Decimal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 * PRECISION / rhs.0)
    }
}

