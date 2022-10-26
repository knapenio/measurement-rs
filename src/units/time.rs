use crate::{Dimension, Unit, UnitConverter};

#[derive(Clone, PartialEq, Debug)]
pub struct Time {
    symbol: &'static str,
    converter: UnitConverter,
}

impl Time {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        Time { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: f64) -> Self {
        Time::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    pub const fn seconds() -> Self {
        Time::with_coeff("s", 1.0)
    }

    pub const fn minutes() -> Self {
        Time::with_coeff("m", 60.0)
    }

    pub const fn hours() -> Self {
        Time::with_coeff("h", 3600.0)
    }
}

impl Unit for Time {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for Time {
    fn base_unit() -> Self {
        Self::seconds()
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
        let seconds = Time::seconds();
        let minutes = Time::minutes();
        let hours = Time::hours();

        assert_eq!(seconds.converter().convert_to_base_unit(1.0), 1.0);
        assert_eq!(minutes.converter().convert_to_base_unit(1.0), 60.0);
        assert_eq!(hours.converter().convert_to_base_unit(1.0), 3600.0);
    }
}
