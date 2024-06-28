use crate::address_offset;
use embedded_hal::i2c::{ErrorType, I2c};

pub enum Command<'command> {
    WriteData(&'command [u8]),
    SetColumnAddress(u16),
}

impl Command<'_> {
    pub fn send<Bus>(
        &self,
        mut bus: Bus,
        base_address: u8,
    ) -> Result<(), <Bus as ErrorType>::Error>
    where
        Bus: I2c,
    {
        match self {
            Self::WriteData(data) => {
                bus.write(base_address + address_offset::WRITE_DATA, data)
            }
            Self::SetColumnAddress(addr) => {
                bus.write(base_address + address_offset::WRITE_COMMAND, todo!())
            }
        }
    }
}
