pub struct Size {
    pub width: u16,
    pub height: u16,
};

pub struct Terminal {
    pub size: Size,
};

impl Terminal {
    pub fn default(&self) -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&slef) -> &Size {
        &self.size
    }
}
