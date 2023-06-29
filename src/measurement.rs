// based on https://github.com/apple/swift-corelibs-foundation/blob/main/Sources/Foundation/Measurement.swift

use super::{Dimension, Unit, Value};
use std::{cmp::Ordering, fmt, ops};

/// A numeric quantity labeled with a unit of measure, with support for unit conversion and unit-aware calculations.
#[derive(Clone, PartialEq)]
pub struct Measurement<U: Unit> {
    pub value: Value,
    unit: U,
}

impl<U: Unit> Copy for Measurement<U> where U: Copy {}

impl<U: Unit> Measurement<U> {
    #[inline]
    pub const fn new(value: Value, unit: U) -> Self {
        Measurement { value, unit }
    }

    pub fn unit(&self) -> &U {
        &self.unit
    }

    /// Computes the absolute value of `self`.
    pub fn abs(&self) -> Self {
        Measurement::new(self.value.abs(), self.unit.clone())
    }
}

impl<U: Unit> Measurement<U>
where
    U: Dimension,
{
    pub fn converted_to(&self, unit: U) -> Self {
        if unit == self.unit {
            Measurement::new(self.value, unit)
        } else {
            let base_value = self.unit.converter().convert_to_base_unit(self.value);
            if unit == U::base_unit() {
                Measurement::new(base_value, unit)
            } else {
                let value = unit.converter().convert_from_base_unit(base_value);
                Measurement::new(value, unit)
            }
        }
    }
}

impl<U: Unit> fmt::Debug for Measurement<U>
where
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Measurement")
            .field("value", &self.value)
            .field("unit", &self.unit)
            .finish()
    }
}

impl<U: Unit> ops::Add for Measurement<U>
where
    U: Dimension,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.unit == other.unit {
            Measurement::new(self.value.add(other.value), self.unit)
        } else {
            let base_value = self.unit.converter().convert_to_base_unit(self.value);
            let other_base_value = other.unit.converter().convert_to_base_unit(other.value);
            Measurement::new(base_value.add(other_base_value), U::base_unit())
        }
    }
}

impl<U: Unit> ops::Sub for Measurement<U>
where
    U: Dimension,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.unit == other.unit {
            Measurement::new(self.value.sub(other.value), self.unit)
        } else {
            let base_value = self.unit.converter().convert_to_base_unit(self.value);
            let other_base_value = other.unit.converter().convert_to_base_unit(other.value);
            Measurement::new(base_value.sub(other_base_value), U::base_unit())
        }
    }
}

impl<U: Unit> ops::Mul<Value> for Measurement<U>
where
    U: Dimension,
{
    type Output = Self;

    fn mul(self, other: Value) -> Self {
        Measurement::new(self.value.mul(other), self.unit)
    }
}

impl<U: Unit> ops::Div<Value> for Measurement<U>
where
    U: Dimension,
{
    type Output = Self;

    fn div(self, other: Value) -> Self {
        Measurement::new(self.value.div(other), self.unit)
    }
}

impl<U: Unit> ops::AddAssign for Measurement<U>
where
    U: Dimension,
{
    fn add_assign(&mut self, other: Self) {
        let other = other.converted_to(self.unit.clone());
        self.value.add_assign(other.value);
    }
}

impl<U: Unit> ops::SubAssign for Measurement<U>
where
    U: Dimension,
{
    fn sub_assign(&mut self, other: Self) {
        let other = other.converted_to(self.unit.clone());
        self.value.sub_assign(other.value);
    }
}

impl<U: Unit> ops::Neg for Measurement<U> {
    type Output = Self;

    fn neg(self) -> Self {
        Measurement::new(self.value.neg(), self.unit)
    }
}

impl<U: Unit> PartialOrd for Measurement<U>
where
    U: Dimension,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit == other.unit {
            self.value.partial_cmp(&other.value)
        } else {
            let base_value = self.unit.converter().convert_to_base_unit(self.value);
            let other_base_value = other.unit.converter().convert_to_base_unit(other.value);
            base_value.partial_cmp(&other_base_value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnitConverter;
    use approx::assert_relative_eq;

    #[derive(PartialEq, Clone, Debug)]
    struct LinearUnit(Value, Value);

    #[test]
    fn convert_to_same_unit_is_eq() {
        let unit = LinearUnit(2.3, 3.2);
        let measurement = Measurement::new(103.5, unit.clone());
        let converted = measurement.converted_to(unit.clone());
        assert_eq!(converted, measurement);
    }

    #[test]
    fn convert_to_same_unit() {
        let unit = LinearUnit(2.3, 3.2);
        let measurement = Measurement::new(103.5, unit.clone());
        let converted = measurement.converted_to(unit.clone());
        assert_eq!(&unit, converted.unit());
        assert_eq!(measurement.value, converted.value);
    }

    #[test]
    fn convert_convert_to_base_unit() {
        let unit = LinearUnit(2.3, 3.2);
        let measurement = Measurement::new(103.5, unit.clone());
        let converted = measurement.converted_to(LinearUnit::base_unit());
        assert_eq!(&LinearUnit::base_unit(), converted.unit());
        assert_relative_eq!(241.25, converted.value, epsilon = Value::EPSILON);
    }

    #[test]
    fn add_measurements() {
        let unit = LinearUnit(1.0, 0.0);
        let lhs = Measurement::new(103.5, unit.clone());
        let rhs = Measurement::new(100.2, unit.clone());
        assert_relative_eq!(203.7, (lhs + rhs).value, epsilon = Value::EPSILON);
    }

    #[test]
    fn sub_measurements() {
        let unit = LinearUnit(1.0, 0.0);
        let lhs = Measurement::new(102.4, unit.clone());
        let rhs = Measurement::new(99.8, unit.clone());
        assert_relative_eq!(2.6, (lhs - rhs).value, epsilon = 0.00000000000001);
    }

    impl Unit for LinearUnit {
        fn symbol(&self) -> String {
            "test".to_owned()
        }
    }

    impl Dimension for LinearUnit {
        fn base_unit() -> LinearUnit {
            LinearUnit(1.0, 0.0)
        }

        fn converter(&self) -> UnitConverter {
            UnitConverter::Linear {
                coeff: self.0,
                constant: self.1,
            }
        }
    }

    #[test]
    fn abs_measurements() {
        let unit = LinearUnit(1.0, 0.0);
        assert_eq!(Measurement::new(-1.0, unit.clone()).abs().value, 1.0);
        assert_eq!(Measurement::new(1.0, unit.clone()).abs().value, 1.0);
        assert_eq!(Measurement::new(-0.0, unit.clone()).abs().value, 0.0);
        assert_eq!(Measurement::new(0.0, unit.clone()).abs().value, 0.0);
    }
}
