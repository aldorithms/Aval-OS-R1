use crate::block::BlockDevice;
use crate::ext4::Ext4FileSystem;
use crate::vfs::FileSystem;

pub enum FileSystems 
{
    Ext4(Ext4FileSystem),
    // add other filesystem drivers here
}

pub fn open_file_system<T: BlockDevice>(device: T, filesystem_type: &str,) -> Result<Box<dyn FileSystem>, FileSystemError> 
{
    match filesystem_type 
    {
        "ext4" => Ok(Box::new(Ext4FileSystem::new(device)?)),
        // add other filesystem drivers here
        _ => Err(FileSystemError::UnsupportedFileSystem),
    }
}
