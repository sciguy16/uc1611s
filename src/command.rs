use crate::address_offset;
use embedded_hal::i2c::{ErrorType, I2c};

pub enum Command<'command> {
    WriteData(&'command [u8]),
    SetColumnAddress(u8),
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
                let low_nybble = addr & 0x0f;
                let high_nybble = addr >> 4;
                let cmd = [low_nybble, 0x10 | high_nybble];

                bus.write(base_address + address_offset::WRITE_COMMAND, &cmd)
            }
        }
    }
}
