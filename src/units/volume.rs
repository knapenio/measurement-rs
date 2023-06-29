use crate::{Dimension, Unit, UnitConverter, Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Volume {
    symbol: &'static str,
    converter: UnitConverter,
}

impl Volume {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        Volume { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: Value) -> Self {
        Volume::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn liters() -> Self {
        Volume::with_coeff("L", 1.0)
    }

    pub const fn gallons() -> Self {
        Volume::with_coeff("gal", 3.78541)
    }
}

impl Unit for Volume {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for Volume {
    fn base_unit() -> Self {
        Self::liters()
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
        let liters = Volume::liters();
        let gallons = Volume::gallons();

        assert_eq!(liters.converter().convert_to_base_unit(1.0), 1.0);
        assert_eq!(gallons.converter().convert_to_base_unit(1.0), 3.78541);
    }
}
