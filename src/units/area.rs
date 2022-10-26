use crate::{Dimension, Unit, UnitConverter};

#[derive(Clone, PartialEq, Debug)]
pub struct Area {
    symbol: &'static str,
    converter: UnitConverter,
}

impl Area {
    pub const fn new(symbol: &'static str, converter: UnitConverter) -> Self {
        Area { symbol, converter }
    }

    const fn with_coeff(symbol: &'static str, coeff: f64) -> Self {
        Area::new(
            symbol,
            UnitConverter::Linear {
                coeff,
                constant: 0.0,
            },
        )
    }

    /// The "square millimeters" (mm2) unit of length.
    pub const fn square_millimeters() -> Self {
        Area::with_coeff("mm2", 1.0)
    }

    /// The "square meters" (m2) unit of length.
    pub const fn square_meters() -> Self {
        Area::with_coeff("m2", 1e6)
    }
}

impl Unit for Area {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

impl Dimension for Area {
    fn base_unit() -> Self {
        Area::square_millimeters()
    }

    fn converter(&self) -> UnitConverter {
        self.converter.clone()
    }
}
