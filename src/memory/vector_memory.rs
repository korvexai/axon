
#[derive(Debug, Default)]
pub struct VectorMemory {
    pub storage: std::collections::HashMap<String, Vec<f32>>,
}

impl VectorMemory {
    pub fn new() -> Self { Self::default() }
}
