#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub(crate) enum ParseContext {
    Content,
    Code,
}
