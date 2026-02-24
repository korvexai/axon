use anyhow::Result;

pub async fn embed(_endpoint: &str, _model: &str, _text: &str) -> Result<Vec<f32>> {
    // Stub stable embedding
    Ok(vec![0.0; 384])
}
