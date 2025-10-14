use crate::codegen::types::Span;
use crate::types::Location;

#[derive(Clone, Debug)]
pub(in crate::codegen) enum Block<'a> {
    KCODE(Span<'a>),
    KCOMMENT(Span<'a>),
    KCONTENT(Span<'a>),
    KFUNCTIONS(Span<'a>),
    KINLINEDCODE(Span<'a>),
    KINLINEDCONTENT(Span<'a>),
    KLAYOUT(Span<'a>),
    KROOT(Span<'a>),
    KRENDER(Span<'a>),
    KSECTION(String, Span<'a>),
    KUSE(Span<'a>),
}

impl<'a> Block<'a> {
    pub(in crate::codegen) fn location(&self) -> Location {
        match self {
            Block::KCODE(span) => span.location(),
            Block::KCOMMENT(span) => span.location(),
            Block::KCONTENT(span) => span.location(),
            Block::KFUNCTIONS(span) => span.location(),
            Block::KINLINEDCODE(span) => span.location(),
            Block::KINLINEDCONTENT(span) => span.location(),
            Block::KLAYOUT(span) => span.location(),
            Block::KRENDER(span) => span.location(),
            Block::KROOT(span) => span.location(),
            Block::KSECTION(_, span) => span.location(),
            Block::KUSE(span) => span.location(),
        }
    }

    pub(in crate::codegen) fn new_code(span: Span<'a>) -> Self {
        Block::KCODE(span)
    }

    pub(in crate::codegen) fn new_comment(span: Span<'a>) -> Self {
        Block::KCOMMENT(span)
    }

    pub(in crate::codegen) fn new_content(span: Span<'a>) -> Self {
        Block::KCONTENT(span)
    }

    pub(in crate::codegen) fn new_functions(span: Span<'a>) -> Self {
        Block::KFUNCTIONS(span)
    }

    pub(in crate::codegen) fn new_inline_code(span: Span<'a>) -> Self {
        Block::KINLINEDCODE(span)
    }

    pub(in crate::codegen) fn new_inline_content(span: Span<'a>) -> Self {
        Block::KINLINEDCONTENT(span)
    }

    pub(in crate::codegen) fn new_layout(span: Span<'a>) -> Self {
        Block::KLAYOUT(span)
    }

    pub(in crate::codegen) fn new_render(span: Span<'a>) -> Self {
        Block::KRENDER(span)
    }

    pub(in crate::codegen) fn new_root(span: Span<'a>) -> Self {
        Block::KROOT(span)
    }

    pub(in crate::codegen) fn new_section(name: &str, span: Span<'a>) -> Self {
        Block::KSECTION(name.to_string(), span)
    }

    pub(in crate::codegen) fn new_use(span: Span<'a>) -> Self {
        Block::KUSE(span)
    }

    pub(in crate::codegen) fn to_content(&self) -> Self {
        match self {
            Block::KSECTION(_, span) => Block::KCONTENT(span.clone()),
            _ => {
                panic!("Only section block can have name");
            }
        }
    }

    pub(in crate::codegen) fn content(&self) -> String {
        match self {
            Block::KCODE(span) => span.content(),
            Block::KCOMMENT(span) => span.content(),
            Block::KCONTENT(span) => span.content(),
            Block::KFUNCTIONS(span) => span.content(),
            Block::KINLINEDCODE(span) => span.content(),
            Block::KINLINEDCONTENT(span) => span.content(),
            Block::KLAYOUT(span) => span.content(),
            Block::KRENDER(span) => span.content(),
            Block::KROOT(span) => span.content(),
            Block::KSECTION(_, span) => span.content(),
            Block::KUSE(span) => span.content(),
        }
    }
}
