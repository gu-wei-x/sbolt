use super::{DisplayOption, DisplayOptionRef, Writer};
use std::fmt::Display;

/// A value that can be streamed into a template's writer.
pub trait WriteTo {
    fn write_to(&self, writer: &mut impl Writer);
}

// Generated dispatch prefers WriteTo over streaming Display.
#[doc(hidden)]
pub struct WriteToRef<'a, T: ?Sized>(pub &'a T);

impl<T: WriteTo + ?Sized> WriteToRef<'_, T> {
    pub fn write_to<W: Writer>(self, mut writer: W) -> W {
        self.0.write_to(&mut writer);
        writer
    }
}

#[doc(hidden)]
pub trait DisplayWriteTo {
    fn write_to<W: Writer>(self, writer: W) -> W;
}

impl<T: Display + ?Sized> DisplayWriteTo for WriteToRef<'_, T> {
    fn write_to<W: Writer>(self, mut writer: W) -> W {
        writer.write_formatted(format_args!("{}", self.0));
        writer
    }
}

impl WriteTo for str {
    fn write_to(&self, writer: &mut impl Writer) {
        writer.write(self);
    }
}

impl WriteTo for String {
    fn write_to(&self, writer: &mut impl Writer) {
        writer.write(self);
    }
}

impl WriteTo for bool {
    fn write_to(&self, writer: &mut impl Writer) {
        writer.write(if *self { "true" } else { "false" });
    }
}

impl<T: WriteTo + ?Sized> WriteTo for &T {
    fn write_to(&self, writer: &mut impl Writer) {
        T::write_to(self, writer);
    }
}

impl<T: WriteTo + ?Sized> WriteTo for &mut T {
    fn write_to(&self, writer: &mut impl Writer) {
        T::write_to(self, writer);
    }
}

macro_rules! write_integers {
    ($($integer:ty),* $(,)?) => {
        $(
            impl WriteTo for $integer {
                fn write_to(&self, writer: &mut impl Writer) {
                    writer.write(itoa::Buffer::new().format(*self));
                }
            }
        )*
    };
}

write_integers!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

/// Explicitly force `Display` formatting instead of a specialized renderer.
pub struct DisplayValue<T>(pub T);

pub fn display<T: Display>(value: T) -> DisplayValue<T> {
    DisplayValue(value)
}

impl<T: Display> Display for DisplayValue<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<T: Display> WriteTo for DisplayValue<T> {
    fn write_to(&self, writer: &mut impl Writer) {
        writer.write_formatted(format_args!("{}", self.0));
    }
}

impl<T: Display + WriteTo> WriteTo for DisplayOption<T> {
    fn write_to(&self, writer: &mut impl Writer) {
        if let Some(value) = &self.0 {
            value.write_to(writer);
        }
    }
}

impl<T: Display + WriteTo> WriteTo for DisplayOptionRef<'_, T> {
    fn write_to(&self, writer: &mut impl Writer) {
        if let Some(value) = self.0 {
            value.write_to(writer);
        }
    }
}
