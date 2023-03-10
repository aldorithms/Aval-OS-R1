use ext4_rs::{BlockDevice, FileSystem, FileSystemError, FileSystemResult, Inode};

pub struct Ext4FileSystem 
{
    fs: FileSystem,
}

impl Ext4FileSystem 
{
    pub fn new<T: BlockDevice>(device: T) -> FileSystemResult<Ext4FileSystem> 
    {
        let fs = FileSystem::new(device)?;
        Ok(Ext4FileSystem { fs })
    }

    pub fn open_inode(&mut self, inode_num: u32) -> FileSystemResult<Inode> 
    {
        self.fs.open_inode(inode_num)
    }
}
