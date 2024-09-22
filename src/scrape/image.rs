use std::io::{Cursor, Read, Seek, SeekFrom};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct ImageMeta {
    pub width: u32,
    pub height: u32,
    pub mime_type: String,
}

impl ImageMeta {
    pub fn ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
    pub fn try_from_bytes(bytes: &Vec<u8>) -> Result<Self> {
        get_image_meta(bytes)
    }
}

pub fn get_image_meta(bytes: &Vec<u8>) -> Result<ImageMeta> {
    let mut cursor = Cursor::new(bytes);
    let mut buffer = [0u8; 3];
    cursor.read_exact(&mut buffer)?;
    let [c1, c2, c3] = buffer;

    fn read_u16(cursor: &mut Cursor<&Vec<u8>>) -> Result<u16> {
        let mut buffer = [0u8; 2];
        cursor.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn read_u32(cursor: &mut Cursor<&Vec<u8>>) -> Result<u32> {
        let mut buffer = [0u8; 4];
        cursor.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    fn read_u32_le(cursor: &mut Cursor<&Vec<u8>>) -> Result<u32> {
        let mut buffer = [0u8; 4];
        cursor.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    // GIF
    if c1 == 0x47 && c2 == 0x49 && c3 == 0x46 {
        cursor.seek(SeekFrom::Current(3))?;
        let width = read_u16(&mut cursor)?;
        let height = read_u16(&mut cursor)?;
        return Ok(ImageMeta { width: width as u32, height: height as u32, mime_type: "image/gif".to_string() });
    }

    // JPG
    if c1 == 0xFF && c2 == 0xD8 {
        let mut c3 = 0xFF;
        while c3 == 0xFF {
            cursor.read_exact(&mut buffer[..1])?;
            c3 = buffer[0];
            let marker = c3;
            let len = read_u16(&mut cursor)?;
            if marker == 192 || marker == 193 || marker == 194 {
                cursor.seek(SeekFrom::Current(1))?;
                let height = read_u16(&mut cursor)?;
                let width = read_u16(&mut cursor)?;
                return Ok(ImageMeta { width: width as u32, height: height as u32, mime_type: "image/jpeg".to_string() });
            }
            cursor.seek(SeekFrom::Current((len - 2) as i64))?;
        }
    }

    // PNG
    if c1 == 137 && c2 == 80 && c3 == 78 {
        cursor.seek(SeekFrom::Current(15))?;
        let width = read_u32(&mut cursor)?;
        cursor.seek(SeekFrom::Current(2))?;
        let height = read_u32(&mut cursor)?;
        return Ok(ImageMeta { width, height, mime_type: "image/png".to_string() });
    }

    // BMP
    if c1 == 66 && c2 == 77 {
        cursor.seek(SeekFrom::Current(15))?;
        let width = read_u32_le(&mut cursor)?;
        cursor.seek(SeekFrom::Current(2))?;
        let height = read_u32_le(&mut cursor)?;
        return Ok(ImageMeta { width, height, mime_type: "image/bmp".to_string() });
    }

    // WEBP
    if c1 == 0x52 && c2 == 0x49 && c3 == 0x46 {
        let mut bytes = [0u8; 27];
        cursor.read_exact(&mut bytes)?;
        let width = ((bytes[24] as u32) << 8) | (bytes[23] as u32);
        let height = ((bytes[26] as u32) << 8) | (bytes[25] as u32);
        return Ok(ImageMeta { width, height, mime_type: "image/webp".to_string() });
    }

    Err(anyhow!("Unsupported Image"))
}