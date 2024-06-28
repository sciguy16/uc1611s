use embedded_hal::i2c::{ErrorType, I2c};

// can't use the display-interface crate trait because the UC1611S
// requires a single-byte read after particular commands, and
// display-interface doesn't currently support reading

pub mod command;
use command::Command;

mod address_offset {
    pub const WRITE_COMMAND: u8 = 0;
    pub const READ_STATUS: u8 = 1;
    pub const WRITE_DATA: u8 = 2;
    pub const READ_DATA: u8 = 3;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressOption {
    A = 0x70,
    B = 0x74,
    C = 0x7C,
    D = 0x78,
}

pub struct UC1611S<Bus> {
    bus: Bus,
    base_address: u8,
}

impl<Bus> UC1611S<Bus>
where
    Bus: I2c,
{
    pub fn new(bus: Bus) -> Self {
        Self::new_custom_address(bus, AddressOption::A as u8)
    }

    pub fn new_with_address(bus: Bus, address: AddressOption) -> Self {
        Self::new_custom_address(bus, address as u8)
    }

    pub fn new_custom_address(bus: Bus, base_address: u8) -> Self {
        assert!(
            base_address < 0x7f - 4,
            "The base address must be at least 4 lower than the maximum",
        );
        Self { bus, base_address }
    }

    pub fn write_data(
        &mut self,
        data: &[u8],
    ) -> Result<(), <Bus as ErrorType>::Error> {
        Command::WriteData(data).send(&mut self.bus, self.base_address)
    }

    pub fn set_column_address(
        &mut self,
        column_address: u16,
    ) -> Result<(), <Bus as ErrorType>::Error> {
        self.bus
            .write(self.base_address + address_offset::WRITE_COMMAND, &[])
    }
}
