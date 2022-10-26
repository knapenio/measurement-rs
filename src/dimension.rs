use super::{Unit, UnitConverter};

pub trait Dimension: Unit {
    fn base_unit() -> Self;
    fn converter(&self) -> UnitConverter;
}
