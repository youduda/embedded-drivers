#![no_std]

use core::fmt;
use core::str;

pub mod bme280;
pub mod bmp180;
pub mod esp_at;
pub mod pmsx003;

pub mod lm75;

pub mod onewire;

/// A fmt::Write for bytes.
pub struct ByteMutWriter<'a> {
    buf: &'a mut [u8],
    cursor: usize,
}

impl<'a> ByteMutWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        ByteMutWriter { buf, cursor: 0 }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.buf[0..self.cursor]) }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn clear(&mut self) {
        self.cursor = 0;
    }

    pub fn len(&self) -> usize {
        self.cursor
    }

    pub fn empty(&self) -> bool {
        self.cursor == 0
    }

    pub fn full(&self) -> bool {
        self.capacity() == self.cursor
    }

    pub fn write_byte(&mut self, b: u8) {
        if !self.full() {
            self.buf[self.cursor] = b;
            self.cursor += 1;
        }
    }

    pub fn write_char(&mut self, c: char) {
        self.write_byte(c as u8);
    }
}

impl fmt::Write for ByteMutWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let cap = self.capacity();
        for (i, &b) in self.buf[self.cursor..cap]
            .iter_mut()
            .zip(s.as_bytes().iter())
        {
            *i = b;
        }
        self.cursor = usize::min(cap, self.cursor + s.as_bytes().len());
        Ok(())
    }
}
