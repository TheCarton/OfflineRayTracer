use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone)]
pub struct PPM {
    pub(crate) height: u32,
    pub(crate) width: u32,
    pub(crate) data: Vec<u8>,
}

impl PPM {
    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        file.write(&self.data)?;
        Ok(())
    }
}