use crate::{Dimension, Unit, UnitConverter, Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Length {
    symbol: &'static str,
    converter: UnitConverter,
}

impl Length {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        Length { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: Value) -> Self {
        Length::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    /// The "meters" unit of length.
    pub const fn meters() -> Self {
        Length::with_coeff("m", 1.0)
    }

    /// The "inches" unit of length.
    pub const fn inches() -> Self {
        Length::with_coeff("in", 0.0254)
    }

    /// The "feet" unit of length.
    pub const fn feet() -> Self {
        Length::with_coeff("ft", 0.3048)
    }
}

impl Unit for Length {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for Length {
    fn base_unit() -> Self {
        Self::meters()
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
        let meters = Length::meters();
        let inches = Length::inches();

        assert_eq!(meters.converter().convert_to_base_unit(1.0), 1.0);
        assert_eq!(inches.converter().convert_to_base_unit(1.0), 0.0254);
    }
}
