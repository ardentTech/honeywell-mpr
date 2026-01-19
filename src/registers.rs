use bitfields::bitfield;

#[bitfield(u8)]
pub struct Status {
    // LSB first
    #[bits(1, access = ro)]
    math_saturation: u8,
    //
    #[bits(1)]
    _pad: u8,
    // Indicates whether the checksum-based integrity check
    // passed or failed; the memory error status bit is calculated
    // only during the power-up sequence.
    #[bits(1, access = ro)]
    memory_integrity: u8,
    //
    #[bits(2)]
    _pad: u8,
    // Indicates that the data for the last command is not yet
    // available. No new commands are processed if the device is
    // busy.
    #[bits(1, access = ro)]
    busy_flag: u8,
    // Needed for the SPI Mode where the Master reads all zeroes
    // if the device is not powered or in power-on reset (POR).
    #[bits(1, access = ro)]
    power_indication: u8,
    //
    #[bits(1)]
    _pad: u8,
}

impl Status {
    pub fn is_busy(&self) -> bool {
        self.busy_flag() == 1
    }

    pub fn is_powered(&self) -> bool {
        self.power_indication() == 1
    }

    pub fn integrity_test_passed(&self) -> bool {
        self.memory_integrity() == 0
    }

    pub fn math_saturation_occurred(&self) -> bool {
        self.math_saturation() == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::{Status};

    #[test]
    fn is_busy_false() {
        let status = Status::from_bits(0b1101_1111);
        assert!(!status.is_busy());
    }

    #[test]
    fn is_busy_true() {
        let status = Status::from_bits(0b0010_0000);
        assert!(status.is_busy());
    }

    #[test]
    fn is_powered_false() {
        let status = Status::from_bits(0b1011_1111);
        assert!(!status.is_powered());
    }

    #[test]
    fn is_powered_true() {
        let status = Status::from_bits(0b0100_0000);
        assert!(status.is_powered());
    }

    #[test]
    fn integrity_test_passed_false() {
        let status = Status::from_bits(0b0000_0100);
        assert!(!status.integrity_test_passed());
    }

    #[test]
    fn integrity_test_passed_true() {
        let status = Status::from_bits(0b1111_1011);
        assert!(status.integrity_test_passed());
    }

    #[test]
    fn math_saturation_occurred_false() {
        let status = Status::from_bits(0b1111_1110);
        assert!(!status.math_saturation_occurred());
    }

    #[test]
    fn math_saturation_occurred_true() {
        let status = Status::from_bits(0b0000_0001);
        assert!(status.math_saturation_occurred());
    }
}