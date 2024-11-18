pub const REGION_FLASH_ADDR: u32 = 0x0800_0000; // Flash base
pub const REGION_RAM_ADDR: u32 = 0x20000000;  // RAM base
pub const REGION_CCRAM_ADDR: u32 = 0x10000000; // CCRAM base

pub const REGION_FLASH_LEN: u32 = 1024*1024;  // 1024 KB
pub const REGION_RAM_LEN: u32 = 128*1024;  // 128 KB
pub const REGION_CCRAM_LEN: u32 = 64*1024;  // 64 KB

pub const ATTR_READ_WRITE: u32 = 0b011;    // RX
pub const ATTR_READ_EXEC: u32 = 0b101;    // RW
