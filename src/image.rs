pub struct Image {
    pub bytes: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl Image {
    pub fn from_file_with_format(bytes: &[u8], format: Option<image::ImageFormat>) -> Image {
        let img = if let Some(fmt) = format {
            image::load_from_memory_with_format(&bytes, fmt)
                .unwrap_or_else(|e| panic!("{}", e))
                .to_rgba8()
        } else {
            image::load_from_memory(&bytes)
                .unwrap_or_else(|e| panic!("{}", e))
                .to_rgba8()
        };
        let width = img.width() as u16;
        let height = img.height() as u16;
        let bytes = img.into_raw();

        Image {
            width,
            height,
            bytes,
        }
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

}