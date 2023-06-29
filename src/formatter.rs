use super::{Measurement, Unit};

#[derive(Default)]
pub struct Formatter {
    pub precision: usize,
}

impl Formatter {
    pub fn with_precision(precision: usize) -> Self {
        Formatter { precision }
    }

    /// Returns a formatted measurement, e.g. "21.5 °C".
    pub fn format<U>(&self, measurement: &Measurement<U>) -> String
    where
        U: Unit,
    {
        if measurement.value.trunc() == measurement.value {
            format!("{} {}", measurement.value, measurement.unit().symbol())
        } else {
            format!(
                "{:.prec$} {}",
                measurement.value,
                measurement.unit().symbol(),
                prec = self.precision
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units;

    #[test]
    fn format_precision() {
        let precision1 = Formatter::with_precision(1);
        let precision4 = Formatter::with_precision(4);

        assert_eq!(
            precision1.format(&Measurement::<units::Power>::new(
                2.3,
                units::Power::kilowatts()
            )),
            "2.3 kW"
        );
        assert_eq!(
            precision1.format(&Measurement::<units::ElectricCharge>::new(
                5.6,
                units::ElectricCharge::coulombs()
            )),
            "5.6 C"
        );
        assert_eq!(
            precision1.format(&Measurement::<units::Energy>::new(
                0.1,
                units::Energy::kilowatt_hours()
            )),
            "0.1 kWh"
        );

        assert_eq!(
            precision4.format(&Measurement::<units::Power>::new(
                2.34567,
                units::Power::kilowatts()
            )),
            "2.3457 kW"
        );
        assert_eq!(
            precision4.format(&Measurement::<units::ElectricCharge>::new(
                5.23456,
                units::ElectricCharge::coulombs()
            )),
            "5.2346 C"
        );
        assert_eq!(
            precision4.format(&Measurement::<units::Energy>::new(
                0.01234,
                units::Energy::kilowatt_hours()
            )),
            "0.0123 kWh"
        );
    }
}
