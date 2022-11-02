#[derive(Debug, Clone)]
pub enum RotationSystem {
    SRS,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub rotation_system: RotationSystem,
}
