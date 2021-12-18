#![allow(unsafe_code)]

use super::super::{
    binding,
    binding::{LZ4DecStream, LZ4Stream},
};
use crate::{Error, ErrorKind, Result};

use std::{
    mem::{size_of, MaybeUninit},
    os::raw::{c_char, c_int, c_void},
    ptr::NonNull,
};

#[allow(clippy::large_enum_variant)]
enum Stream {
    Stack(LZ4Stream),
    Heap(NonNull<LZ4Stream>),
}

pub struct CompressionContext {
    stream: Stream,
}

unsafe impl Send for CompressionContext {}

impl CompressionContext {
    pub fn new() -> Result<Self> {
        let mut stream = MaybeUninit::<LZ4Stream>::uninit();
        unsafe {
            let ptr = binding::LZ4_initStream(
                stream.as_mut_ptr() as *mut c_void,
                size_of::<LZ4Stream>() as usize,
            );
            if !ptr.is_null() {
                return Ok(Self {
                    stream: Stream::Stack(stream.assume_init()),
                });
            }
            NonNull::new(binding::LZ4_createStream())
        }
        .ok_or_else(|| Error::new(ErrorKind::InitializationFailed))
        .map(|stream| Self {
            stream: Stream::Heap(stream),
        })
    }

    fn get_ptr(&mut self) -> *mut LZ4Stream {
        match &mut self.stream {
            Stream::Stack(stream) => stream as *mut LZ4Stream,
            Stream::Heap(ptr) => ptr.as_ptr(),
        }
    }

    pub fn next(&mut self, src: &[u8], dst: &mut [u8], acceleration: i32) -> usize {
        unsafe {
            binding::LZ4_compress_fast_continue(
                self.get_ptr(),
                src.as_ptr() as *const c_char,
                dst.as_mut_ptr() as *mut c_char,
                src.len() as c_int,
                dst.len() as c_int,
                acceleration as c_int,
            ) as usize
        }
    }

    pub fn load_dict(&mut self, dict: &[u8]) {
        unsafe {
            binding::LZ4_loadDict(
                self.get_ptr(),
                dict.as_ptr() as *const c_char,
                dict.len() as c_int,
            );
        }
    }

    pub fn save_dict(&mut self, dict: &mut [u8]) {
        unsafe {
            binding::LZ4_saveDict(
                self.get_ptr(),
                dict.as_ptr() as *mut c_char,
                dict.len() as c_int,
            );
        }
    }
}

impl Drop for CompressionContext {
    fn drop(&mut self) {
        if let Stream::Heap(mut ptr) = self.stream {
            unsafe {
                binding::LZ4_freeStream(ptr.as_mut());
            }
        }
    }
}

pub struct DecompressionContext {
    stream: NonNull<LZ4DecStream>,
}

unsafe impl Send for DecompressionContext {}

impl DecompressionContext {
    pub fn new() -> Result<Self> {
        unsafe {
            let ptr = NonNull::new(binding::LZ4_createStreamDecode());
            ptr.ok_or_else(|| Error::new(ErrorKind::InitializationFailed))
                .map(|stream| Self { stream })
        }
    }

    pub fn reset(&mut self, dict: &[u8]) -> Result<()> {
        let result = unsafe {
            binding::LZ4_setStreamDecode(
                self.stream.as_ptr(),
                dict.as_ptr() as *const c_char,
                dict.len() as c_int,
            )
        };
        if result == 1 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::InitializationFailed))
        }
    }

    pub fn decompress(&mut self, src: &[u8], dst: &mut [u8]) -> Result<usize> {
        let result = unsafe {
            binding::LZ4_decompress_safe_continue(
                self.stream.as_ptr(),
                src.as_ptr() as *const c_char,
                dst.as_mut_ptr() as *mut c_char,
                src.len() as c_int,
                dst.len() as c_int,
            ) as i32
        };
        if result < 0 {
            Err(Error::new(ErrorKind::DecompressionFailed))
        } else {
            Ok(result as usize)
        }
    }
}

impl Drop for DecompressionContext {
    fn drop(&mut self) {
        unsafe {
            binding::LZ4_freeStreamDecode(self.stream.as_mut());
        }
    }
}
