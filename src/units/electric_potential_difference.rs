use crate::{Dimension, Unit, UnitConverter};

#[derive(Clone, PartialEq, Debug)]
pub struct ElectricPotentialDifference {
    symbol: &'static str,
    converter: UnitConverter,
}

impl ElectricPotentialDifference {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        ElectricPotentialDifference { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: f64) -> Self {
        ElectricPotentialDifference::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn volts() -> Self {
        ElectricPotentialDifference::with_coeff("V", 1.0)
    }
}

impl Unit for ElectricPotentialDifference {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for ElectricPotentialDifference {
    fn base_unit() -> Self {
        Self::volts()
    }

    fn converter(&self) -> UnitConverter {
        self.converter
    }
}
