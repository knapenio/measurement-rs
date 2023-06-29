use crate::{Dimension, Unit, UnitConverter, Value};

#[derive(Clone, PartialEq, Debug)]
pub struct ElectricCharge {
    symbol: &'static str,
    converter: UnitConverter,
}

impl ElectricCharge {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        ElectricCharge { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: Value) -> Self {
        ElectricCharge::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn coulombs() -> Self {
        ElectricCharge::with_coeff("C", 1.0)
    }

    pub const fn ampere_hours() -> Self {
        ElectricCharge::with_coeff("Ah", 3600.0)
    }
}

impl Unit for ElectricCharge {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for ElectricCharge {
    fn base_unit() -> Self {
        Self::coulombs()
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
        let coulombs = ElectricCharge::coulombs();
        let ampere_hours = ElectricCharge::ampere_hours();

        assert_eq!(coulombs.converter().convert_to_base_unit(1.0), 1.0);
        assert_eq!(ampere_hours.converter().convert_to_base_unit(1.0), 3600.0);
    }
}
