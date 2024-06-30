use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::Gray8,
    Pixel,
};
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

impl<Bus> OriginDimensions for UC1611S<Bus> {
    fn size(&self) -> Size {
        Size::new(64, 64)
    }
}

impl<Bus> DrawTarget for UC1611S<Bus> {
    type Color = Gray8;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or greater than
            // (63,63)). `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if let Ok((x @ 0..=63, y @ 0..=63)) = coord.try_into() {
                // Calculate the index in the framebuffer.
                // let index: u32 = x + y * 64;
                // self.framebuffer[index as usize] = color.luma();
                //TODO set column address & write the byte
            }
        }

        Ok(())
    }
}
