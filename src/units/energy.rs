use crate::{Dimension, Unit, UnitConverter};

#[derive(Clone, PartialEq, Debug)]
pub struct Energy {
    symbol: &'static str,
    converter: UnitConverter,
}

impl Energy {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        Energy { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: f64) -> Self {
        Energy::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn joules() -> Self {
        Energy::with_coeff("J", 1.0)
    }

    pub const fn watt_hours() -> Self {
        Energy::with_coeff("Wh", 3600.0)
    }

    pub const fn kilowatt_hours() -> Self {
        Energy::with_coeff("kWh", 3600000.0)
    }
}

impl Unit for Energy {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for Energy {
    fn base_unit() -> Self {
        Self::joules()
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
        let joules = Energy::joules();
        let watt_hours = Energy::watt_hours();
        let kilowatt_hours = Energy::kilowatt_hours();

        assert_eq!(joules.converter().convert_to_base_unit(1.0), 1.0);
        assert_eq!(watt_hours.converter().convert_to_base_unit(1.0), 3600.0);
        assert_eq!(
            kilowatt_hours.converter().convert_to_base_unit(1.0),
            3600000.0
        );
    }
}
