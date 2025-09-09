#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum Kind {
    CODE,
    HTML,
    EOF,
    UNKNOWN,
}