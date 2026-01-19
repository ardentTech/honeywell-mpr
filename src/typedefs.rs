const KPA_IN_PSI: f32 = 6.894757;
const PSI_IN_BAR: f32 = 14.50377;

/// Wraps raw measurement data for easy unit conversions.
pub struct Reading {
    pub(crate) range_min: f32,
    pub(crate) range_max: f32,
    pub raw_data: u32,
    pub(crate) transfer_function: TransferFunction
}
impl Reading {
    pub fn new(range_min: f32, range_max: f32, raw_data: u32, transfer_function: TransferFunction) -> Self {
        Self { range_min, range_max, raw_data, transfer_function }
    }

    /// Converts raw measurement data to bar.
    pub fn bar(&self) -> f32 {
        self.psi() / PSI_IN_BAR
    }

    /// Converts raw measurement data to kPa.
    pub fn kpa(&self) -> f32 {
        self.psi() * KPA_IN_PSI
    }

    /// Converts raw measurement data to PSI.
    pub fn psi(&self) -> f32 {
        ((self.raw_data as f32 - self.transfer_function.min_counts()) * (self.range_max - self.range_min)) /
            (self.transfer_function.max_counts() - self.transfer_function.min_counts()) + self.range_min
    }
}

#[derive(Clone, Copy)]
pub enum TransferFunction {
    /// 10% to 90% of 2**24 counts
    A,
    /// 2.5% to 22.5% of 2**24 counts
    B,
    /// 20% to 80% of 2**24 counts
    C
}
impl TransferFunction {
    pub fn min_counts(&self) -> f32 {
        match self {
            // precomputed percentages
            TransferFunction::A => 1677721.6,
            TransferFunction::B => 419430.4,
            TransferFunction::C => 3355443.3,
        }
    }
    pub fn max_counts(&self) -> f32 {
        match self {
            // precomputed percentages
            TransferFunction::A => 15099494.0,
            TransferFunction::B => 3774873.5,
            TransferFunction::C => 13421773.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::relative_eq;
    use super::*;

    #[test]
    fn reading_bar_ok() {
        let reading = Reading::new(-1.0, 1.0, 14260634, TransferFunction::A);
        let _ = relative_eq!(reading.bar(), 0.0603, epsilon = f32::EPSILON);
    }

    #[test]
    fn reading_kpa_ok() {
        let reading = Reading::new(-1.0, 1.0, 14260634, TransferFunction::A);
        let _ = relative_eq!(reading.kpa(), 6.0329, epsilon = f32::EPSILON);
    }

    #[test]
    fn reading_psi_ok() {
        let reading = Reading::new(-1.0, 1.0, 14260634, TransferFunction::A);
        let _ = relative_eq!(reading.psi(), 0.875, epsilon = f32::EPSILON);
    }
}