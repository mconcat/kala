#[path="./runtime.rs"]
mod runtime;

pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    Number(i64),
    Bigint(i64, i64), // TODO
    String(str),
    
    Reference(Reference),
    Closure(Closure),
}

impl runtime::Value for Value {
    type N = Number;
    type B = Bigint;
    type S = String;
    type R = Reference;
    type C = Closure;
}

pub enum Number {
    NaN,
    Infinity(bool), // true = positive, false = negative
    Integer(i64),
}

use Number::NaN;
use Number::Infinity;
use Number::Integer;

impl Number {
    #[inline]
    fn ok(&self, value: ()) -> Result<&Self, str> {
        Ok(self)
    }

    #[inline]
    fn assign(&mut self, value: Self) -> Result<&Self, str> {
        *self = value;
        Ok(self)
    }

    #[inline]
    fn type_error(&mut self) -> Result<&Self, str> {
        Err("Type error")
    }

    #[inline]
    fn to_int32(&self) -> i32 {
        match self {
            &Integer(i) => i as i32, // TODO
            _ => 0,
        }
    }
}

impl runtime::Number for Number {
    #[inline]
    fn add(&mut self, other: &Self) -> Result<&mut Self, str> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => {
                    if *x == *y {
                        self.ok(())
                    } else {
                        self.assign(NaN)
                    }
                }
                _ => self.ok(()) // ignore other cases
            }
            Integer(x) => match other {
                Infinity(_) => self.assign(other),
                Integer(y) => self.ok(*x = *x + *y),
                NaN => self.assign(NaN),
            },
        }
    }

    #[inline]
    fn sub(&mut self, other: &Self) -> Result<&mut Self, str> {
        _ => match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => {
                    if *x != *y {
                        self.ok(())
                    } else {
                        self.assign(NaN)
                    }
                }
                _ => Ok,
            }
            Integer(x) => match other {
                Infinity(y) => self.ok(*y = !*y),
                Integer(y) => self.ok(*x = *x - *y),
                NaN => self.assign(NaN),
            },
        }
    }

    #[inline]
    fn mul(&mut self, other: &Self) -> Result<&mut Self, str> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => self.ok(*x = *x == *y),
                Integer(0) => self.assign(NaN),
                Integer(y) => self.ok(*x = *x == (y>=0)),
            }
            Integer(0) => match other {
                Infinity(_) => self.assign(NaN),
                NaN => self.assign(NaN),
                _ => self.type_error(),
            },
            Integer(x) => match other {
                Infinity(y) => self.assign(Infinity((x>=0) == y)),
                Integer(y) => self.ok(*x = *x * *y),
                NaN => self.assign(NaN),
                _ => self.type_error(),
            },
        }
    }

    #[inline]
    fn div(&mut self, other: &Self) -> Result<&mut Self, str> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => self.assign(NaN),
                Integer(y) => self.ok(*x = *x == (y>=0)),
                NaN => self.assign(NaN),
            }
            Integer(0) => match other {
                Integer(0) => self.assign(NaN),
                NaN => self.assign(NaN),
                _ => self.ok(()),
            },
            Integer(x) => match other {
                Integer(0) => self.assign(Infinity(x>=0)),
                Infinity(y) => self.ok(*x = 0),
                Integer(y) => self.ok(*x = *x / *y),
                NaN => self.assign(NaN),
                _ => self.type_error(),
            },
        }
    }

    #[inline]
    fn modulo(&mut self, other: &Self) -> Result<&mut Self, str> {
        match self {
            NaN => Ok,
            Infinity(x) => self.assign(NaN),
            Integer(x) => match other {
                Infinity(y) => self.ok(()),
                Integer(0) => self.assign(NaN),
                Integer(y) => self.ok(*x = *x % *y),
                NaN => self.assign(NaN),
                _ => self.type_error(), 
            },
        }
    }

    #[inline]
    fn pow(&mut self, other: &Self) -> Result<&mut Self, str> {
        unimplemented!("asdf")
    } // TODO XXX

    #[inline]
    fn bitand(&mut self, other: &Self) -> Result<&mut Self, str> {
        let v = self.to_int32() & other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v),
            _ => self.assign(Integer(v)),
        }  
    }

    #[inline]
    fn bitor(&mut self, other: &Self) -> Result<&mut Self, str> {
        let v = self.to_int32() | other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v),
            _ => self.assign(Integer(v)),
        } 
    }

    #[inline]
    fn bitxor(&mut self, other: &Self) -> Result<&mut Self, str> {
        let v = self.to_int32() ^ other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v),
            _ => self.assign(Integer(v)),
        }    
    }

    #[inline]
    fn bitnot(&mut self) -> Result<&mut Self, str> {
        let v = self.to_int32() ^ 0xFFFFFFFF;
        match self {
            Integer(x) => self.ok(*x = v),
            _ => self.assign(Integer(v)),
        }
    }

    #[inline]
    fn lshift(&mut self, other: &Self) -> Result<&mut Self, str> {
        let v = self.to_int32() << other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v),
            _ => self.assign(Integer(v)),
        } 
    }

    #[inline]
    fn rshift(&mut self, other: &Self) -> Result<&mut Self, str> {
        let v = self.to_int32() >> other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v),
            _ => self.assign(Integer(v)),
        }   
    }

    #[inline]
    fn urshift(&mut self, other: &Self) -> Result<&mut Self, str> {
        unimplemented!("asdfasdf")
    }

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => true,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x == *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x == *y,
                _ => false,
            },
        }
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    // Copilot wrote, need to check
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x < *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x < *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn le(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x <= *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x <= *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x > *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x > *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x >= *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x >= *y,
                _ => false,
            },
        }
    }
}

pub enum Bigint {
    V128(bool, [u32; 4]),
}

use Bigint::V128;

impl Bigint {
    #[inline]
    fn ok(&self, value: ()) -> Result<&Self, str> {
        Ok(self)
    }

    #[inline]
    fn assign(&mut self, value: Self) -> Result<&Self, str> {
        *self = value;
        Ok(self)
    }

    #[inline]
    fn type_error(&self) -> Result<&Self, str> {
        Err("Type error")
    }

    #[inline]
    fn to_int32(&self) -> i32 {
        match self {
            &Integer(i) => i as i32, // TODO
            _ => 0,
        }
    }
}


impl runtime::Bigint for Bigint {
    #[inline]
    fn add(&mut self, other: &Self) -> Result<&mut Self, str> {
        match self {
            V128(xsign, x) => match other {
                V128(ysign, y) => {
                    let val = x[0] as i64;
                    let carry = 0;
                    val += y[0] * ((xsign != ysign) as i32 * -1);
                    if val < 0 {
                        carry = -1;
                        val = u32::MAX - val;
                    } else if val > u32::MAX {
                        carry = 1;
                        val = val - u32::MAX;
                    }
                    x[0] = val as u32;

                    let val = x[1] as i64;
                    let carry = 0;
                    val += y[0] * ((xsign != ysign) as i32 * -1);
                    if val < 0 {
                        carry = -1;
                        val = u32::MAX - val;
                    } else if val > u32::MAX {
                        carry = 1;
                        val = val - u32::MAX;
                    }
                    x[1] = val as u32;
                }
            },
        }
        self.ok(())
    }
}

// Reference is a pointer to a value in the heap.

pub struct Reference {

}
