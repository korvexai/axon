pub fn chunk_text(content: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < content.len() {
        let end = (start + chunk_size).min(content.len());
        let slice = &content[start..end];
        chunks.push(slice.to_string());

        if end == content.len() {
            break;
        }

        start = end.saturating_sub(overlap);
    }

    chunks
}
