use crate::{Dimension, Unit, UnitConverter, Value};

#[derive(Clone, PartialEq, Debug)]
pub struct ElectricResistance {
    symbol: &'static str,
    converter: UnitConverter,
}

impl ElectricResistance {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        ElectricResistance { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: Value) -> Self {
        ElectricResistance::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn ohms() -> Self {
        ElectricResistance::with_coeff("Ω", 1.0)
    }
}

impl Unit for ElectricResistance {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for ElectricResistance {
    fn base_unit() -> Self {
        Self::ohms()
    }

    fn converter(&self) -> UnitConverter {
        self.converter
    }
}
