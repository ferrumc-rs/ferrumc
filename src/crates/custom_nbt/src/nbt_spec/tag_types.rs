pub type TagId = u8;

pub const TAG_END: TagId = 0;
pub const TAG_BYTE: TagId = 1;
pub const TAG_SHORT: TagId = 2;
pub const TAG_INT: TagId = 3;
pub const TAG_LONG: TagId = 4;
pub const TAG_FLOAT: TagId = 5;
pub const TAG_DOUBLE: TagId = 6;
pub const TAG_BYTE_ARRAY: TagId = 7;
pub const TAG_STRING: TagId = 8;
pub const TAG_LIST: TagId = 9;
pub const TAG_COMPOUND: TagId = 10;
pub const TAG_INT_ARRAY: TagId = 11;
pub const TAG_LONG_ARRAY: TagId = 12;