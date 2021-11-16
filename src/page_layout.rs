use std::mem::size_of;

pub const PTR_SIZE : usize = size_of::<usize>();

// Set page size to 4kb
pub const PAGE_SIZE: usize = 4096;
