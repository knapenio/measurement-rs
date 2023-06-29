use crate::{Dimension, Unit, UnitConverter, Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Power {
    symbol: &'static str,
    converter: UnitConverter,
}

impl Power {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        Power { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: Value) -> Self {
        Power::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn watts() -> Self {
        Power::with_coeff("W", 1.0)
    }

    pub const fn kilowatts() -> Self {
        Power::with_coeff("kW", 1000.0)
    }
}

impl Unit for Power {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for Power {
    fn base_unit() -> Self {
        Self::watts()
    }

    fn converter(&self) -> UnitConverter {
        self.converter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_to_base() {
        let watts = Power::watts();
        let kilowatts = Power::kilowatts();

        assert_eq!(watts.converter().convert_to_base_unit(1.0), 1.0);
        assert_eq!(kilowatts.converter().convert_to_base_unit(1.0), 1000.0);
    }
}
