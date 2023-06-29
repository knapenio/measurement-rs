// based on https://github.com/apple/swift-corelibs-foundation/blob/main/Sources/Foundation/Unit.swift

use crate::Value;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UnitConverter {
    Linear { coeff: Value, constant: Value },
}

impl UnitConverter {
    pub fn convert_to_base_unit(&self, val: Value) -> Value {
        match self {
            UnitConverter::Linear { coeff, constant } => (val * coeff) + constant,
        }
    }

    pub fn convert_from_base_unit(&self, val: Value) -> Value {
        match self {
            UnitConverter::Linear { coeff, constant } => (val - constant) / coeff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let conv = UnitConverter::Linear {
            coeff: 1.0,
            constant: 0.0,
        };

        assert_eq!(0.0, conv.convert_to_base_unit(0.0));
        assert_eq!(1.0, conv.convert_to_base_unit(1.0));
        assert_eq!(-3.0, conv.convert_to_base_unit(-3.0));

        assert_eq!(0.0, conv.convert_from_base_unit(0.0));
        assert_eq!(1.0, conv.convert_from_base_unit(1.0));
        assert_eq!(-3.0, conv.convert_from_base_unit(-3.0));
    }

    #[test]
    fn zero_coeff_and_zero_constant() {
        let conv = UnitConverter::Linear {
            coeff: 0.0,
            constant: 0.0,
        };

        assert_eq!(0.0, conv.convert_to_base_unit(0.0));
        assert_eq!(0.0, conv.convert_to_base_unit(1.0));
        assert_eq!(0.0, conv.convert_to_base_unit(-3.0));

        assert!(conv.convert_from_base_unit(0.0).is_nan());
        let val = conv.convert_from_base_unit(1.0);
        assert!(val.is_infinite() && val.is_sign_positive());
        let val = conv.convert_from_base_unit(-3.0);
        assert!(val.is_infinite() && val.is_sign_negative());
    }

    #[test]
    fn zero_coeff_and_non_zero_constant() {
        let conv = UnitConverter::Linear {
            coeff: 0.0,
            constant: -2.0,
        };

        assert_eq!(-2.0, conv.convert_to_base_unit(0.0));
        assert_eq!(-2.0, conv.convert_to_base_unit(1.0));
        assert_eq!(-2.0, conv.convert_to_base_unit(-3.0));

        assert!(conv.convert_from_base_unit(0.0).is_infinite());
        assert!(conv.convert_from_base_unit(1.0).is_infinite());
        assert!(conv.convert_from_base_unit(-3.0).is_infinite());
    }

    #[test]
    fn non_zero_coeff() {
        let conv = UnitConverter::Linear {
            coeff: 2.0,
            constant: 0.0,
        };

        assert_eq!(0.0, conv.convert_to_base_unit(0.0));
        assert_eq!(2.0, conv.convert_to_base_unit(1.0));
        assert_eq!(-6.0, conv.convert_to_base_unit(-3.0));

        assert_eq!(0.0, conv.convert_from_base_unit(0.0));
        assert_eq!(0.5, conv.convert_from_base_unit(1.0));
        assert_eq!(-1.5, conv.convert_from_base_unit(-3.0));
    }

    #[test]
    fn non_zero_constant() {
        let conv = UnitConverter::Linear {
            coeff: 1.0,
            constant: 3.0,
        };

        assert_eq!(3.0, conv.convert_to_base_unit(0.0));
        assert_eq!(4.0, conv.convert_to_base_unit(1.0));
        assert_eq!(0.0, conv.convert_to_base_unit(-3.0));

        assert_eq!(-3.0, conv.convert_from_base_unit(0.0));
        assert_eq!(-2.0, conv.convert_from_base_unit(1.0));
        assert_eq!(-6.0, conv.convert_from_base_unit(-3.0));
    }

    #[test]
    fn non_zero_coeff_and_constant() {
        let conv = UnitConverter::Linear {
            coeff: 2.0,
            constant: 3.0,
        };

        assert_eq!(3.0, conv.convert_to_base_unit(0.0));
        assert_eq!(5.0, conv.convert_to_base_unit(1.0));
        assert_eq!(-3.0, conv.convert_to_base_unit(-3.0));

        assert_eq!(-1.5, conv.convert_from_base_unit(0.0));
        assert_eq!(-1.0, conv.convert_from_base_unit(1.0));
        assert_eq!(-3.0, conv.convert_from_base_unit(-3.0));
    }
}
