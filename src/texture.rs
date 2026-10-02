use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Texture {
    pub fn load_bmp(path: &str) -> Result<Self, String> {
        let mut file = File::open(path).map_err(|e| format!("No se pudo abrir {}: {}", path, e))?;

        let mut header = [0u8; 54];

        file.read_exact(&mut header)
            .map_err(|e| format!("BMP invalido {}: {}", path, e))?;

        if header[0] != b'B' || header[1] != b'M' {
            return Err(format!("{} no es un archivo BMP valido", path));
        }

        let data_offset =
            u32::from_le_bytes([header[10], header[11], header[12], header[13]]) as u64;

        let width = i32::from_le_bytes([header[18], header[19], header[20], header[21]]);

        let height = i32::from_le_bytes([header[22], header[23], header[24], header[25]]);

        let bits_per_pixel = u16::from_le_bytes([header[28], header[29]]);

        if bits_per_pixel != 32 {
            return Err(format!(
                "{} debe ser BMP de 32 bits. Tiene {} bits.",
                path, bits_per_pixel
            ));
        }

        let width_abs = width.unsigned_abs() as usize;

        let height_abs = height.unsigned_abs() as usize;

        file.seek(SeekFrom::Start(data_offset))
            .map_err(|e| e.to_string())?;

        let mut pixels = vec![0u32; width_abs * height_abs];

        let mut pixel = [0u8; 4];

        for file_y in 0..height_abs {
            let destination_y = if height > 0 {
                height_abs - 1 - file_y
            } else {
                file_y
            };

            for x in 0..width_abs {
                file.read_exact(&mut pixel).map_err(|e| e.to_string())?;

                let b = pixel[0] as u32;
                let g = pixel[1] as u32;
                let r = pixel[2] as u32;

                pixels[destination_y * width_abs + x] = (r << 16) | (g << 8) | b;
            }
        }

        Ok(Self {
            width: width_abs,
            height: height_abs,
            pixels,
        })
    }

    pub fn sample(&self, u: f32, v: f32) -> u32 {
        let u = u.rem_euclid(1.0);

        let v = v.clamp(0.0, 0.9999);

        let x = (u * self.width as f32) as usize;

        let y = (v * self.height as f32) as usize;

        self.pixels[y * self.width + x]
    }
}
