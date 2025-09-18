#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Kind<'a> {
    CODE(&'a str),
    CONTENT(&'a str),
}

pub(crate) type Fragment<'a> = crate::codegen::parser::Span<Kind<'a>>;
