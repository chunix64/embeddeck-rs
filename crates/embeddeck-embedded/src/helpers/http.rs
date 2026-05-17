pub fn unchunk(chunked: &str) -> &str {
    // Skip first chunk size line (e.g. "1ae\r\n")
    if let Some(start) = chunked.find("\r\n") {
        let start = start + 2;

        // Cut before the final "0\r\n"
        if let Some(end) = chunked.rfind("\r\n0\r\n") {
            return &chunked[start..end];
        }

        // Fallback
        return chunked[start..].trim_end_matches(|c: char| c == '\r' || c == '\n');
    }

    chunked
}
