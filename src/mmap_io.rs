//! .

use memmap2::{Mmap, MmapMut};
use std::{fs::File, io, path::Path};

/// Immutable memory-mapped file reader
pub struct MmapReader {
    inner: Mmap,
}

impl MmapReader {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self { inner: mmap })
    }

    pub fn from_file(file: &File) -> io::Result<Self> {
        let mmap = unsafe { Mmap::map(file)? };
        Ok(Self { inner: mmap })
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Mutable memory-mapped file writer
pub struct MmapWriter {
    inner: MmapMut,
}

impl MmapWriter {
    pub fn open<P: AsRef<Path>>(path: P, size: usize) -> io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(path)?;

        file.set_len(size as u64)?;
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self { inner: mmap })
    }

    pub fn from_file(file: &File) -> io::Result<Self> {
        let mmap = unsafe { MmapMut::map_mut(file)? };
        Ok(Self { inner: mmap })
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
