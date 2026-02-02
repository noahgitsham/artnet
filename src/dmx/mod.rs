pub struct Dmx {
    universe: [u8; 512]
}

impl Dmx {
    pub fn new() -> Self {
        Dmx {
            universe: [0; 512],
        }
    }
}
