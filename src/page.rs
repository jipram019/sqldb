
// Value is a wrapper of value in a page
pub struct Value(usize);

// Page is a representation of single page of memory
pub struct Page {
    data: Box<[u8; PAGE_SIZE]>;
}

impl Page {
    pub fn new(data: [u8; PAGE_SIZE]) -> Page {
        Page {
            data: Box::new(data),
        }
    }
}