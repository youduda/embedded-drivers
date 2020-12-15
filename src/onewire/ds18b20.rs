/// DS18B20 digital thermometer.
pub struct DS18B20;

/// Initiates a single temperature conversion.
pub const CMD_CONVERT_T: u8 = 0x44;
/// Writes 3 bytes of data to the DS18B20’s scratchpad.
pub const CMD_WRITE_SCRATCHPAD: u8 = 0x4e;
/// Reads the contents of the scratchpad.
pub const CMD_READ_SCRATCHPAD: u8 = 0xbe;
/// Copies the contents of the scratchpad T_H, T_L and configuration registers
/// (bytes 2, 3 and 4) to EEPROM.
pub const CMD_COPY_SCRATCHPAD: u8 = 0x48;
/// Recalls the alarm trigger values and configuration data from EEPROM and
/// places the data in bytes 2, 3, and 4, respectively, in the scratchpad memory.
pub const CMD_RECALL_EEPROM: u8 = 0xb8;
/// Determines if any DS18B20s on the bus are using parasite power.
pub const CMD_POWER_SUPPLY: u8 = 0xb4;

#[repr(u8)]
pub enum Resolution {
    Nine = 0b000_11111,
    Ten = 0b001_11111,
    Eleven = 0b010_11111,
    Twelve = 0b011_11111,
}

impl Resolution {
    pub fn bits(&self) -> u8 {
        match self {
            Resolution::Nine => 9,
            Resolution::Ten => 10,
            Resolution::Eleven => 11,
            Resolution::Twelve => 12,
        }
    }

    pub fn conversion_time_ms(&self) -> u16 {
        match self {
            Resolution::Nine => 94,
            Resolution::Ten => 188,
            Resolution::Eleven => 375,
            Resolution::Twelve => 750,
        }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::Twelve
    }
}
