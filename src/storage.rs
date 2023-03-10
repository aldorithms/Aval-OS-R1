pub trait StorageDevice 
{
    fn read_sector(&mut self, sector: u64, buf: &mut [u8]) -> Result<(), &'static str>;
    fn write_sector(&mut self, sector: u64, buf: &[u8]) -> Result<(), &'static str>;
}
