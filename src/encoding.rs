//! Минимальное RFC 3986-кодирование, используемое только внутри клиента.

/// Percent-encode a path segment.
pub(crate) fn path_segment(value: &str) -> String {
    encode(value, false)
}

/// Encode a query key or value according to RFC 3986.
pub(crate) fn query_component(value: &str) -> String {
    encode(value, true)
}

/// Build a query string from already selected optional pairs.
pub(crate) fn query_string(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(key, value)| format!("{}={}", query_component(key), query_component(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn encode(value: &str, keep_slash: bool) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric()
            || matches!(byte, b'-' | b'.' | b'_' | b'~')
            || (keep_slash && byte == b'/')
        {
            encoded.push(byte as char);
        } else {
            const HEX: &[u8; 16] = b"0123456789ABCDEF";
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_reserved_and_unicode_query_characters() {
        assert_eq!(query_component("a b&ёж"), "a%20b%26%D1%91%D0%B6");
    }

    #[test]
    fn encodes_a_path_segment_without_allowing_slashes() {
        assert_eq!(path_segment("user/name"), "user%2Fname");
    }
}
