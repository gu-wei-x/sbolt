use crate::types::Writer;

pub struct ViewContext<'a, W>
where
    W: Writer + 'static + ?Sized,
{
    writer: &'a mut W,
}

impl<'a, W> ViewContext<'a, W>
where
    W: Writer + 'static + ?Sized,
{
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    pub fn write(&mut self, content: &str) {
        self.writer.write(content);
    }

    pub fn writeln(&mut self, content: &str) {
        self.writer.write(content);
    }
}
