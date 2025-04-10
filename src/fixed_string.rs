//! A no_std compatible implementation of the `String` type, for testing purposes.
//!
//! This implementation is not intended to be used in production, but rather to
//! provide a simple and easy to understand implementation of the `String` type
//! for testing purposes.

const BUFFER_SIZE: usize = 256;

#[derive(Clone, PartialEq, Eq)]
pub struct String {
    /// The internal buffer that stores the string data
    buffer: [u8; BUFFER_SIZE],
    /// The current length of the string (may be less than buffer capacity)
    size: usize,
}

impl String {
    pub const fn new() -> Self {
        Self {
            buffer: [0; BUFFER_SIZE],
            size: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.size
    }

    pub const fn available_space(&self) -> usize {
        BUFFER_SIZE - self.len()
    }

    pub fn push_str(&mut self, s: &str) {
        let current_len = self.len();
        if s.len() > self.available_space() {
            panic!("not enough space in buffer");
        }

        let src = s.as_bytes();
        let dst = &mut self.buffer[current_len..current_len + s.len()];
        dst.copy_from_slice(src);
        self.size += s.len();
    }

    pub fn push(&mut self, c: char) {
        let mut buf = [0; 4];
        let encoded = c.encode_utf8(&mut buf);
        self.push_str(encoded)
    }

    /// Returns a string slice containing the entire string
    pub fn as_str(&self) -> &str {
        // Safety: We ensure the buffer only contains valid UTF-8 data
        // by validating all inputs through push_str and push methods
        unsafe { str::from_utf8_unchecked(&self.buffer[..self.len()]) }
    }

    /// Returns a mutable string slice containing the entire string
    pub fn as_mut_str(&mut self) -> &mut str {
        let len = self.len();
        // Safety: Same safety guarantees as as_str
        unsafe { str::from_utf8_unchecked_mut(&mut self.buffer[..len]) }
    }
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}

impl ToString for i32 {
    fn to_string(&self) -> String {
        let mut string = String::new();

        // special case for zero
        if *self == 0 {
            string.push('0');
            return string;
        }

        // handle negative numbers
        let mut value = *self;
        let negative = value < 0;

        // special case for min val to avoid overflow
        if value == i32::MIN {
            return "-2147483648".to_string();
        }

        // make positive
        if negative {
            value = -value;
        }

        // convert the number to a temp buffer, in reverse order
        let mut buffer = [0u8; 10]; // max 10 digits for i32
        let mut length = 0;

        while value > 0 {
            buffer[length] = (value % 10) as u8 + b'0';
            value /= 10;
            length += 1;
        }

        // make sure we have enough space in the buffer
        let total_length = length + if negative { 1 } else { 0 };
        if total_length > BUFFER_SIZE {
            panic!("buffer too small for i32 representation");
        }

        // add the sign if needed
        if negative {
            string.push('-');
        }

        // add digits in correct order (reversing our buffer)
        for i in (0..length).rev() {
            string.push(buffer[i] as char);
        }

        string
    }
}

impl ToString for &'static str {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for c in self.chars() {
            string.push(c);
        }
        string
    }
}

impl std::ops::Deref for String {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

// Implement DerefMut for mutable access
impl std::ops::DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}

// Display implementation for easy printing
impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// Debug implementation
impl std::fmt::Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("String")
            .field("len", &self.len())
            .field("content", &self.as_str())
            .finish()
    }
}

// Implement From<&str> for convenient construction
impl TryFrom<&str> for String {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut string = String::new();
        string.push_str(s);
        Ok(string)
    }
}
