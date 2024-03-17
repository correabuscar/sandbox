use std::ops::Add;

use num_rational::BigRational;
//use num_bigint::BigUint;
extern crate num_bigint as bigint;
use bigint::ToBigInt;

#[derive(Debug, PartialEq, Clone)] //Debug needed for assert_eq!() , Clone to avoid implicit Copy
struct Millimeters(f64);

#[derive(Debug, Clone)] //needed for assert_eq!()
struct Meters(f64);

#[derive(Debug, Clone, PartialEq)] //Debug needed for assert_eq!()
pub struct GoodMeters {
    inner: BigRational, // using field inner to show it can be done with named fields too
}

#[derive(Debug, Clone, PartialEq)] //Debug needed for assert_eq!()
pub struct GoodMillimeters(BigRational); //or it can be done without named fields

impl GoodMillimeters {
    fn new(num: u64) -> GoodMillimeters {
        GoodMillimeters(BigRational::from_integer(num.to_bigint().unwrap()))
    }

    //TODO: use traits to get same-name functions like this: https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name  but unsure how this would work for non-empty structs!(eg. it would still have to be inited somehow, initially)
    fn new2(bi: BigRational) -> GoodMillimeters {
        GoodMillimeters(bi)
    }
}

impl GoodMeters {
    fn new(num: u64) -> GoodMeters {
        GoodMeters {
            inner: BigRational::from_integer(num.to_bigint().unwrap()),
        }
    }

    fn new2(bi: BigRational) -> GoodMeters {
        GoodMeters { inner: bi }
    }
}

impl Add<GoodMeters> for GoodMillimeters {
    type Output = GoodMillimeters;

    fn add(self, other: GoodMeters) -> GoodMillimeters {
        //GoodMillimeters(self.0 + (other.inner * 1000_u16))
        #[allow(clippy::suspicious_arithmetic_impl)]
        let a: BigRational = other.inner * 1000.to_bigint().unwrap();
        GoodMillimeters::new2(self.0 + a)
        //GoodMillimeters::new(self.0 + (1000 * other.inner))
    }
}

impl Add<GoodMillimeters> for GoodMeters {
    type Output = GoodMeters;

    fn add(self, other: GoodMillimeters) -> GoodMeters {
        //GoodMillimeters(self.0 + (other.inner * 1000_u16))
        #[allow(clippy::suspicious_arithmetic_impl)]
        let a: BigRational = other.0 / 1000.to_bigint().unwrap();
        GoodMeters::new2(self.inner + a)
        //GoodMillimeters::new(self.0 + (1000 * other.inner))
    }
}

impl PartialEq<GoodMillimeters> for GoodMeters {
    fn eq(&self, other: &GoodMillimeters) -> bool {
        self.inner.clone() * 1000.to_bigint().unwrap() == other.0
    }
}

impl PartialEq<GoodMeters> for GoodMillimeters {
    fn eq(&self, other: &GoodMeters) -> bool {
        self.0 == other.inner.clone() * 1000.to_bigint().unwrap()
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000_f64))
    }
}

impl PartialEq<Meters> for Millimeters {
    fn eq(&self, other: &Meters) -> bool {
        self.0 == other.0 * 1000_f64
    }
}

//XXX: or I can just #[derive(PartialEq)]
//impl PartialEq for Millimeters {
//    fn eq(&self, other: &Self) -> bool {
//        self.0 == other.0
//    }
//}

impl Add<Millimeters> for Meters {
    type Output = Meters;

    fn add(self, other: Millimeters) -> Meters {
        Meters(self.0 + (other.0 / 1000_f64))
    }
}

impl PartialEq for Meters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Millimeters> for Meters {
    fn eq(&self, other: &Millimeters) -> bool {
        self.0 * 1000_f64 == other.0
    }
}

//TODO: sebk's idea:
//impl<U: Into<Meters>> PartialEq<U> for Millimeters {
//}

fn main() {
    assert_eq!(Meters(1_f64), Millimeters(1000_f64));
    assert_eq!(GoodMeters::new(1), GoodMillimeters::new(1000));
    assert_eq!(Millimeters(1000_f64), Meters(1_f64));
    assert_eq!(GoodMillimeters::new(1000), GoodMeters::new(1));
    assert_eq!(
        Millimeters(1000_f64) + Meters(1_f64),
        Meters(1_f64) + Millimeters(1000_f64)
    );
    assert_eq!(
        GoodMillimeters::new(1000) + GoodMeters::new(1),
        GoodMeters::new(1) + GoodMillimeters::new(1000)
    );
    let mm = Millimeters(10_f64); //11mm here fails(with 1m), due to lossy f64
    assert_eq!(mm, mm);
    let m = Meters(3_f64); //2m here fails(with 10mm), due to lossy f64
    assert_eq!(m, m);
    let added = mm.clone() + m.clone();
    let added2 = m + mm;
    assert_eq!(added, added2);
    assert_eq!(added2, added);

    let gm = GoodMeters::new(2);
    assert_eq!(gm, gm);
    let gmm = GoodMillimeters::new(10);
    assert_eq!(gmm, gmm);
    let added3 = gm.clone() + gmm.clone();
    let added4 = gmm + gm;
    assert_eq!(added3, added4);
    assert_eq!(added4, added3);

    assert_eq!(
        GoodMeters::new(2) + GoodMillimeters::new(10),
        GoodMillimeters::new(10) + GoodMeters::new(2)
    ); // this is fine
       //assert_eq!(Meters(2_f64)+Millimeters(10_f64), Millimeters(10_f64)+Meters(2_f64));//XXX: yes this will fail due to lossy f64
}
