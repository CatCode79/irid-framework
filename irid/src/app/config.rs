/// Application's configuration.

//= CONFIG STRUCT ==================================================================================

pub struct Config {
    pub clear_color: wgpu::Color,
}


impl Config {
    ///
    // TODO: da fare con serde
    pub fn new(_filename: &String) -> Self {
        Config::default()
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            clear_color: wgpu::Color::WHITE,
        }
    }
}
