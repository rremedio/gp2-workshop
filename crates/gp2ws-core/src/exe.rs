use std::path::Path;

pub struct ExeImage {
    pub bytes: Vec<u8>,
}

impl ExeImage {
    pub fn from_bytes(b: Vec<u8>) -> Self {
        Self { bytes: b }
    }
    /// Bounds-checked little-endian read of `width` bytes at `off`.
    /// Returns `None` if `[off, off+width)` is out of range.
    pub fn try_read(&self, off: usize, width: u8) -> Option<u64> {
        let end = off.checked_add(width as usize)?;
        let s = self.bytes.get(off..end)?;
        let mut v = 0u64;
        for (i, &b) in s.iter().enumerate() {
            v |= (b as u64) << (8 * i);
        } // little-endian
        Some(v)
    }
    /// Bounds-checked little-endian write of `width` bytes at `off`.
    /// Returns `false` (writing nothing) if `[off, off+width)` is out of range.
    pub fn try_write(&mut self, off: usize, width: u8, val: u64) -> bool {
        let Some(end) = off.checked_add(width as usize) else {
            return false;
        };
        let Some(s) = self.bytes.get_mut(off..end) else {
            return false;
        };
        for (i, b) in s.iter_mut().enumerate() {
            *b = ((val >> (8 * i)) & 0xFF) as u8;
        }
        true
    }
    /// Little-endian read of `width` bytes at `off`.
    ///
    /// # Panics
    /// Panics if `[off, off+width)` is out of range. Callers must ensure the
    /// offset is in bounds (e.g. via calibration); use [`try_read`] otherwise.
    pub fn read(&self, off: usize, width: u8) -> u64 {
        debug_assert!(
            off.checked_add(width as usize).map_or(false, |e| e <= self.bytes.len()),
            "ExeImage::read out of bounds: off={off} width={width} len={}",
            self.bytes.len()
        );
        self.try_read(off, width).unwrap_or_else(|| {
            panic!(
                "ExeImage::read out of bounds: off={off} width={width} len={}",
                self.bytes.len()
            )
        })
    }
    /// Little-endian write of `width` bytes at `off`.
    ///
    /// # Panics
    /// Panics if `[off, off+width)` is out of range. Callers must ensure the
    /// offset is in bounds (e.g. via calibration); use [`try_write`] otherwise.
    pub fn write(&mut self, off: usize, width: u8, val: u64) {
        debug_assert!(
            off.checked_add(width as usize).map_or(false, |e| e <= self.bytes.len()),
            "ExeImage::write out of bounds: off={off} width={width} len={}",
            self.bytes.len()
        );
        let len = self.bytes.len();
        if !self.try_write(off, width, val) {
            panic!("ExeImage::write out of bounds: off={off} width={width} len={len}");
        }
    }
    pub fn load(path: &Path) -> std::io::Result<Self> {
        Ok(Self::from_bytes(std::fs::read(path)?))
    }
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, &self.bytes)
    }
    /// copy to `<path>.bak` if no backup exists yet
    pub fn backup(path: &Path) -> std::io::Result<()> {
        let bak = path.with_extension("bak");
        if !bak.exists() {
            std::fs::copy(path, &bak)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_write_u16_le() {
        let mut img = ExeImage::from_bytes(vec![0; 16]);
        img.write(4, 2, 0x48A0);
        assert_eq!(img.bytes[4], 0xA0);
        assert_eq!(img.bytes[5], 0x48);
        assert_eq!(img.read(4, 2), 0x48A0);
    }
    #[test]
    fn read_u32_le() {
        let img = ExeImage::from_bytes(vec![0, 0, 0, 0, 0x00, 0x00, 0x04, 0x00]);
        assert_eq!(img.read(4, 4), 0x40000);
    }

    #[test]
    fn try_read_in_bounds() {
        let img = ExeImage::from_bytes(vec![0xA0, 0x48, 0, 0]);
        assert_eq!(img.try_read(0, 2), Some(0x48A0));
    }

    #[test]
    fn try_read_oob_returns_none() {
        let img = ExeImage::from_bytes(vec![0u8; 4]);
        // Offset past the end.
        assert_eq!(img.try_read(4, 1), None);
        // Straddling the end.
        assert_eq!(img.try_read(3, 2), None);
        // Overflowing offset arithmetic.
        assert_eq!(img.try_read(usize::MAX, 2), None);
    }

    #[test]
    fn try_write_in_bounds() {
        let mut img = ExeImage::from_bytes(vec![0u8; 4]);
        assert!(img.try_write(0, 2, 0x48A0));
        assert_eq!(img.bytes[0], 0xA0);
        assert_eq!(img.bytes[1], 0x48);
    }

    #[test]
    fn try_write_oob_returns_false_and_does_not_write() {
        let mut img = ExeImage::from_bytes(vec![0u8; 4]);
        assert!(!img.try_write(4, 1, 0xFF));
        assert!(!img.try_write(3, 2, 0xFFFF));
        assert!(!img.try_write(usize::MAX, 2, 0xFFFF));
        // Nothing was written anywhere.
        assert_eq!(img.bytes, vec![0u8; 4]);
    }
}
