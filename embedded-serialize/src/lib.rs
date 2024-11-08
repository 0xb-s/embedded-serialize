#![no_std]

#[cfg(feature = "esp32")]
pub mod platform {
    pub fn platform_info() -> &'static str {
        "ESP32 Platform"
    }
}

#[cfg(feature = "arduino32")]
pub mod platform {
    pub fn platform_info() -> &'static str {
        "Arduino32 Platform"
    }
}
use core::mem::size_of;

/// Serialize data to bytes
pub trait Serialize {
    /// Serializes the data into the provided buffer.
    /// Returns the number of bytes written or an error if the buffer is too small.
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError>;
}

/// Deserialize data from bytes
pub trait Deserialize: Sized {
    /// Deserializes the data from the provided buffer.
    /// Returns the instance of the type if successful or an error.
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError>;
}

/// Errors that can occur during serialization
#[derive(Debug)]
pub enum SerializeError {
    /// Buffer provided is too small
    BufferTooSmall,
    /// Custom error variant for future extensions
    Custom(&'static str),
}

/// Errors that can occur during deserialization
#[derive(Debug)]
pub enum DeserializeError {
    /// Buffer provided is too small
    BufferTooSmall,
    /// Data is invalid or corrupted
    InvalidData,
    /// Custom error variant for future extensions
    Custom(&'static str),
}

impl Serialize for u8 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        if buf.len() < 1 {
            return Err(SerializeError::BufferTooSmall);
        }
        buf[0] = *self;
        Ok(1)
    }
}

impl Deserialize for u8 {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        if buf.len() < 1 {
            return Err(DeserializeError::BufferTooSmall);
        }
        Ok(buf[0])
    }
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        if buf.len() < 2 {
            return Err(SerializeError::BufferTooSmall);
        }
        buf[0] = (*self >> 8) as u8;
        buf[1] = *self as u8;
        Ok(2)
    }
}

impl Deserialize for u16 {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        if buf.len() < 2 {
            return Err(DeserializeError::BufferTooSmall);
        }
        Ok(((buf[0] as u16) << 8) | (buf[1] as u16))
    }
}

impl Serialize for u32 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        if buf.len() < 4 {
            return Err(SerializeError::BufferTooSmall);
        }
        buf[0] = (*self >> 24) as u8;
        buf[1] = (*self >> 16) as u8;
        buf[2] = (*self >> 8) as u8;
        buf[3] = *self as u8;
        Ok(4)
    }
}

impl Deserialize for u32 {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        if buf.len() < 4 {
            return Err(DeserializeError::BufferTooSmall);
        }
        Ok(((buf[0] as u32) << 24)
            | ((buf[1] as u32) << 16)
            | ((buf[2] as u32) << 8)
            | (buf[3] as u32))
    }
}

impl Serialize for i8 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        if buf.len() < 1 {
            return Err(SerializeError::BufferTooSmall);
        }
        buf[0] = *self as u8;
        Ok(1)
    }
}

impl Deserialize for i8 {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        if buf.len() < 1 {
            return Err(DeserializeError::BufferTooSmall);
        }
        Ok(buf[0] as i8)
    }
}

impl Serialize for i16 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        let u_val: u16 = (*self) as u16;
        u_val.serialize(buf)
    }
}

impl Deserialize for i16 {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        let u_val = u16::deserialize(buf)?;
        Ok(u_val as i16)
    }
}


impl Serialize for i32 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        let u_val: u32 = (*self) as u32;
        u_val.serialize(buf)
    }
}

impl Deserialize for i32 {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        let u_val = u32::deserialize(buf)?;
        Ok(u_val as i32)
    }
}


impl Serialize for bool {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        if buf.is_empty() {
            return Err(SerializeError::BufferTooSmall);
        }
        buf[0] = if *self { 1 } else { 0 };
        Ok(1)
    }
}

impl Deserialize for bool {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        if buf.is_empty() {
            return Err(DeserializeError::BufferTooSmall);
        }
        match buf[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DeserializeError::InvalidData),
        }
    }
}


impl<T: Serialize, const N: usize> Serialize for [T; N] {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, SerializeError> {
        let mut total = 0;
        for item in self.iter() {
            let size = item.serialize(&mut buf[total..])?;
            total += size;
        }
        Ok(total)
    }
}

impl<T: Deserialize, const N: usize> Deserialize for [T; N] {
    fn deserialize(buf: &[u8]) -> Result<Self, DeserializeError> {
        let mut array: [core::mem::MaybeUninit<T>; N] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut offset = 0;
        for i in 0..N {
            let item = T::deserialize(&buf[offset..])?;
            offset += size_of::<T>();
            array[i] = core::mem::MaybeUninit::new(item);
        }
   
        let array = unsafe { core::mem::transmute_copy::<_, [T; N]>(&array) };
        Ok(array)
    }
}
