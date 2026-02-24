//! OCR module using Burn deep learning framework
//!
//! This module provides OCR functionality using PaddleOCR models converted
//! to native Burn code via burn-onnx. When the `models` feature is enabled
//! and the ONNX models are present during build, full OCR functionality is
//! available. Otherwise, a stub implementation is provided.

use image::{DynamicImage, RgbImage};
use std::path::Path;

/// A detected text region with its content and confidence score
#[derive(Debug, Clone)]
pub struct TextRegion {
    pub text: String,
    pub confidence: f32,
}

/// OCR pipeline using Burn-converted PaddleOCR models
///
/// This struct provides text detection and recognition using models
/// that were converted from ONNX format to native Burn code at build time.
pub struct BurnOCR {
    _dictionary: Vec<String>,
}

impl BurnOCR {
    /// Create a new OCR pipeline with the converted Burn models
    ///
    /// # Arguments
    /// * `dictionary_path` - Path to the character dictionary file
    ///
    /// # Returns
    /// Returns an OCR pipeline instance, or an error if initialization fails.
    ///
    /// # Note
    /// This requires the `models` feature to be enabled and the ONNX models
    /// to be present during build time.
    pub fn new(dictionary_path: &str) -> Result<Self, String> {
        let dict_content = std::fs::read_to_string(dictionary_path)
            .map_err(|e| format!("Failed to read dictionary: {}", e))?;
        let dictionary: Vec<String> = dict_content.lines().map(|s| s.to_string()).collect();

        Ok(Self {
            _dictionary: dictionary,
        })
    }

    /// Run OCR on an RGB image
    ///
    /// # Arguments
    /// * `_image` - The RGB image to process
    ///
    /// # Returns
    /// Returns a vector of detected text regions with their content and confidence.
    ///
    /// # Note
    /// Currently returns a placeholder error. Full implementation requires
    /// the burn-converted models to be built with the ONNX files present.
    pub fn predict(&self, _image: RgbImage) -> Result<Vec<TextRegion>, String> {
        // Note: Full implementation requires the models feature to be enabled
        // and ONNX models to be present during build.
        //
        // The burn-based OCR pipeline would:
        // 1. Preprocess the image for text detection
        // 2. Run the detection model to find text regions
        // 3. For each region, run the recognition model
        // 4. Decode the output using CTC decoding with the dictionary
        //
        // For now, return an error indicating models need to be built
        Err("OCR models not available. Build with ONNX models present.".to_string())
    }
}

/// Load an image from a path and convert to RgbImage
#[allow(dead_code)]
pub fn load_image(path: &Path) -> Result<RgbImage, String> {
    let img = image::open(path).map_err(|e| format!("Failed to open image: {}", e))?;
    Ok(img.to_rgb8())
}

/// Convert a DynamicImage to RgbImage
pub fn dynamic_to_rgb(image: DynamicImage) -> RgbImage {
    image.to_rgb8()
}
