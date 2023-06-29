use crate::{Dimension, Unit, UnitConverter, Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Temperature {
    symbol: &'static str,
    converter: UnitConverter,
}

impl Temperature {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        Temperature { symbol, converter }
    }

    const fn with_coeff_and_constant(symbol: &'static str, coeff: Value, constant: Value) -> Self {
        Temperature::new(symbol, UnitConverter::Linear { coeff, constant })
    }

    pub const fn kelvin() -> Self {
        Temperature::with_coeff_and_constant("K", 1.0, 0.0)
    }

    pub const fn celsius() -> Self {
        Temperature::with_coeff_and_constant("°C", 1.0, 273.15)
    }

    pub const fn fahrenheit() -> Self {
        Temperature::with_coeff_and_constant("°F", 0.55555555555556, 255.37222222222427)
    }
}

impl Unit for Temperature {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for Temperature {
    fn base_unit() -> Self {
        Self::kelvin()
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
        let kelvin = Temperature::kelvin();
        let celsius = Temperature::celsius();
        let fahrenheit = Temperature::fahrenheit();

        assert_eq!(kelvin.converter().convert_to_base_unit(1.0), 1.0);
        assert_eq!(celsius.converter().convert_to_base_unit(1.0), 274.15);
        assert_eq!(
            fahrenheit.converter().convert_to_base_unit(1.0),
            255.92777777777985
        );
    }
}
