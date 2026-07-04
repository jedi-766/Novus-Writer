//! Compression utilities for .notes format

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use crate::models::error::AppError;

/// Compress data using gzip
pub fn compress(data: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;
    Ok(compressed)
}

/// Decompress gzip data
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

/// Compress and base64 encode
pub fn compress_and_encode(data: &[u8]) -> Result<String, AppError> {
    let compressed = compress(data)?;
    Ok(base64::encode(&compressed))
}

/// Base64 decode and decompress
pub fn decode_and_decompress(encoded: &str) -> Result<Vec<u8>, AppError> {
    let decoded = base64::decode(encoded)?;
    decompress(&decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let original = b"Hello, World! This is a test of compression.";
        let compressed = compress(original).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(original, decompressed.as_slice());
    }

    #[test]
    fn test_compress_and_encode() {
        let original = b"Test data for encoding";
        let encoded = compress_and_encode(original).unwrap();
        let decoded = decode_and_decompress(&encoded).unwrap();
        assert_eq!(original, decoded.as_slice());
    }
}
