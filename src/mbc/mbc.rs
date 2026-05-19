mod nombc;
mod mbc1;
mod mbc3;

#[derive(PartialEq)]
pub enum BankingMode
{
    Simple,
    Advanced
}

pub trait MBC
{
    fn readRom(&self, address: u16) -> u8;
    fn writeRom(&mut self, address: u16, value: u8);

    fn readRam(&self, address: u16) -> u8;
    fn writeRam(&mut self, address: u16, value: u8);

    fn saveRam(&self);
    fn loadSave(&mut self);
}