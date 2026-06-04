use std::path::Path;

pub struct ExeImage {
    pub bytes: Vec<u8>,
}

impl ExeImage {
    pub fn from_bytes(b: Vec<u8>) -> Self {
        Self { bytes: b }
    }
    pub fn read(&self, off: usize, width: u8) -> u64 {
        let s = &self.bytes[off..off + width as usize];
        let mut v = 0u64;
        for (i, &b) in s.iter().enumerate() {
            v |= (b as u64) << (8 * i);
        } // little-endian
        v
    }
    pub fn write(&mut self, off: usize, width: u8, val: u64) {
        for i in 0..width as usize {
            self.bytes[off + i] = ((val >> (8 * i)) & 0xFF) as u8;
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
}
