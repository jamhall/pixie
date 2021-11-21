pub struct ApplicationConfig {
    width: u32,
    height: u32,
    address: String,
}

#[allow(dead_code)]
impl ApplicationConfig {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn split(&self) -> (u32, u32, String) {
        (self.width, self.height, self.address.clone())
    }
}

pub struct ApplicationConfigBuilder {
    width: u32,
    height: u32,
    address: String,
}

impl ApplicationConfigBuilder {
    pub fn new() -> Self {
        ApplicationConfigBuilder {
            width: 960,
            height: 240,
            address: "127.0.0.1:8796".to_string(),
        }
    }

    pub fn build(&self) -> ApplicationConfig {
        ApplicationConfig {
            width: self.width,
            height: self.height,
            // ugly...
            address: self.address.clone(),
        }
    }

    pub fn width(&mut self, value: u32) -> &mut Self {
        self.width = value;
        self
    }

    pub fn height(&mut self, value: u32) -> &mut Self {
        self.height = value;
        self
    }

    pub fn address(&mut self, value: String) -> &mut Self {
        self.address = value;
        self
    }
}
