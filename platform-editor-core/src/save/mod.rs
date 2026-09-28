use std::io::{Cursor, Read, Result, Write};

pub mod campaign;

pub const LEVEL_SAVE_LOCATION: &str = "level_save.bin";

/// A trait to write a type into something implementing [`Write`].
pub trait WriteTo {
    fn write(&self, writer: &mut impl Write) -> Result<()>;
}

/// A trait to read a type from a [`Cursor`].
pub trait ReadFrom: Sized {
    fn read(data: &mut Cursor<&[u8]>) -> Result<Self>;
}

/// A trait to read a type from a [`Cursor`].
pub trait VersionedReadFrom: Sized {
    type Version;

    fn read(data: &mut Cursor<&[u8]>, version: Self::Version) -> Result<Self>;
}

pub trait SaveFile: Sized {
    type Version: WriteTo + ReadFrom;

    fn current_version() -> Self::Version;

    fn write_content(&self, writer: &mut impl Write) -> Result<()>;

    fn read_content(version: Self::Version, data: &mut Cursor<&[u8]>) -> Result<Self>;
}

impl<T: SaveFile> WriteTo for T {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        // Save the version.
        Self::current_version().write(writer)?;

        // Save the content.
        self.write_content(writer)
    }
}

impl<T: SaveFile> ReadFrom for T {
    fn read(data: &mut Cursor<&[u8]>) -> Result<Self> {
        // Read the version.
        let version = <<Self as SaveFile>::Version>::read(data)?;

        // Read the content.
        Self::read_content(version, data)
    }
}

macro_rules! number_impls {
    ( $( $ty:ty )+ ) => {
        $(
            impl WriteTo for $ty {
                fn write(&self, writer: &mut impl Write) -> Result<()> {
                    writer.write_all(&self.to_be_bytes())
                }
            }

            impl ReadFrom for $ty {
                fn read(data: &mut Cursor<&[u8]>) -> Result<Self> {
                    let mut buf = [0; size_of::<Self>()];
                    data.read_exact(&mut buf)?;
                    Ok(Self::from_le_bytes(buf))
                }
            }
        )+
    };
}

number_impls!(
    i8 i16 i32 i64
    u8 u16 u32 u64 usize
    f32 f64
);

impl WriteTo for bool {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        u8::from(*self).write(writer)
    }
}

impl ReadFrom for bool {
    fn read(data: &mut Cursor<&[u8]>) -> Result<Self> {
        Ok(u8::read(data)? != 0)
    }
}

impl<T: ReadFrom, const N: usize> ReadFrom for [T; N] {
    fn read(data: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut vec = Vec::with_capacity(N);
        for _ in 0..N {
            vec.push(T::read(data)?);
        }

        // SAFETY: vec has N elements.
        Ok(unsafe { Self::try_from(vec).unwrap_unchecked() })
    }
}
