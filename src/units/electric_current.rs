use crate::{Dimension, Unit, UnitConverter, Value};

#[derive(Clone, PartialEq, Debug)]
pub struct ElectricCurrent {
    symbol: &'static str,
    converter: UnitConverter,
}

impl ElectricCurrent {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        ElectricCurrent { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: Value) -> Self {
        ElectricCurrent::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn amperes() -> Self {
        ElectricCurrent::with_coeff("A", 1.0)
    }
}

impl Unit for ElectricCurrent {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for ElectricCurrent {
    fn base_unit() -> Self {
        Self::amperes()
    }

    fn converter(&self) -> UnitConverter {
        self.converter
    }
}
