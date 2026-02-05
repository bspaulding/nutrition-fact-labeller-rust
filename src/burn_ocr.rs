use burn::tensor::{Tensor, TensorData};
use burn::backend::ndarray::NdArray;
use image::RgbImage;
use log::debug;

use crate::MyTextRegion;

// Type alias for the backend we're using - NdArray for complete operation support
// Note: Candle backend doesn't support adaptive_avg_pool2d used in these ONNX models
// Note: LibTorch backend requires external libtorch C++ library
// NdArray is slower but has full operation coverage and no external dependencies
type B = NdArray<f32>;

/// Preprocess an RGB image for the detection model
/// The detection model expects input shape [1, 3, H, W] with values normalized to [0, 1]
fn preprocess_image_for_detection(img: &RgbImage) -> Tensor<B, 4> {
    let (width, height) = img.dimensions();
    debug!("Preprocessing image: {}x{}", width, height);
    
    // Resize image to a standard size (e.g., 960x960) for detection
    // This is typical for DBNet-style detection models
    let target_size = 960;
    let img_resized = image::imageops::resize(
        img,
        target_size,
        target_size,
        image::imageops::FilterType::Lanczos3,
    );
    
    // Create data in NCHW format [batch, channels, height, width]
    let mut data_vec = Vec::with_capacity(1 * 3 * target_size as usize * target_size as usize);
    
    // Fill in the data in NCHW format (all of channel 0, then all of channel 1, then channel 2)
    for c in 0..3 {
        for y in 0..target_size {
            for x in 0..target_size {
                let pixel = img_resized.get_pixel(x, y);
                let value = match c {
                    0 => pixel[0] as f32 / 255.0,
                    1 => pixel[1] as f32 / 255.0,
                    _ => pixel[2] as f32 / 255.0,
                };
                data_vec.push(value);
            }
        }
    }
    
    // Convert to Burn tensor with explicit shape specification
    // This ensures proper NCHW layout interpretation
    let device = Default::default();
    
    // Create TensorData with explicit shape specification
    let shape_vec = vec![1, 3, target_size as usize, target_size as usize];
    debug!("Creating detection tensor with shape: {:?}, data length: {}", shape_vec, data_vec.len());
    let tensor_data = TensorData::new(data_vec, shape_vec);
    
    let tensor = Tensor::<B, 4>::from_data(tensor_data.convert::<f32>(), &device);
    debug!("Detection tensor shape: {:?}", tensor.shape());
    tensor
}

/// Post-process detection model output to extract text regions
/// The detection model outputs a probability map that needs to be thresholded and contoured
fn postprocess_detection(output: Tensor<B, 4>) -> Vec<(u32, u32, u32, u32)> {
    debug!("Post-processing detection output");
    
    // Get the output shape: [batch, channels, height, width]
    let output_data = output.to_data();
    let shape = output_data.shape.clone();
    
    if shape[0] != 1 {
        debug!("Warning: Expected batch size 1, got {}", shape[0]);
        return vec![];
    }
    
    let height = shape[2];
    let width = shape[3];
    
    // Convert tensor data to vector
    let values: Vec<f32> = output_data.as_slice::<f32>()
        .map(|s| s.to_vec())
        .unwrap_or_default();
    
    if values.is_empty() {
        debug!("Warning: Empty detection output");
        return vec![];
    }
    
    // Simple thresholding approach
    // In a full implementation, this would:
    // 1. Apply threshold to create binary map
    // 2. Find connected components / contours
    // 3. Filter by size and aspect ratio
    // 4. Apply Non-Maximum Suppression (NMS)
    
    // For now, use a simple sliding window approach to find high-probability regions
    let threshold = 0.5;
    let mut regions = Vec::new();
    
    // Scan the probability map for regions above threshold
    // Use larger windows to ensure regions are big enough for recognition model
    let window_size = 80;  // Increased from 64 to ensure minimum size for recognition
    let stride = 40;       // Increased stride proportionally
    let min_region_size = 64;  // Minimum width/height for a valid region (increased from 48)
    
    for y in (0..height).step_by(stride) {
        for x in (0..width).step_by(stride) {
            let mut sum = 0.0;
            let mut count = 0;
            
            for dy in 0..window_size.min(height - y) {
                for dx in 0..window_size.min(width - x) {
                    let idx = (y + dy) * width + (x + dx);
                    if idx < values.len() {
                        sum += values[idx];
                        count += 1;
                    }
                }
            }
            
            let avg = if count > 0 { sum / count as f32 } else { 0.0 };
            
            if avg > threshold {
                let w = window_size.min(width - x) as u32;
                let h = window_size.min(height - y) as u32;
                
                // Only add regions that meet minimum size requirements
                if w >= min_region_size && h >= min_region_size {
                    regions.push((x as u32, y as u32, w, h));
                }
            }
        }
    }
    
    // Merge overlapping regions (simple approach)
    if regions.is_empty() {
        // Fallback: return one large region covering most of the image
        debug!("No regions detected, using fallback region");
        // Fallback region: ensure reasonable dimensions that won't overflow after resizing
        // Limit width to avoid tensor overflow issues in pooling operations
        let fallback_w = ((width as u32).saturating_sub(100).max(150)).min(640);  // Cap at 640 pixels
        let fallback_h = ((height as u32).saturating_sub(100).max(80)).min(320);   // Cap at 320 pixels
        vec![(50, 50, fallback_w, fallback_h)]
    } else {
        debug!("Detected {} regions", regions.len());
        regions
    }
}

/// Preprocess a text region for the recognition model
fn preprocess_region_for_recognition(
    img: &RgbImage,
    bbox: (u32, u32, u32, u32),
) -> Tensor<B, 4> {
    let (x, y, w, h) = bbox;
    debug!("Preprocessing region: ({}, {}, {}, {})", x, y, w, h);
    
    // Crop the region from the original image
    let cropped = image::imageops::crop_imm(img, x, y, w, h).to_image();
    
    // Resize to recognition model input size (typically 32 height, variable width)
    let target_height = 32;
    let aspect_ratio = w as f32 / h as f32;
    let target_width = (target_height as f32 * aspect_ratio) as u32;
    // Ensure minimum width of 100 to avoid pooling issues, clamp maximum to 320
    // PaddleOCR recognition models use multiple pooling layers that require larger minimum dimensions
    let target_width = target_width.max(100).min(320); // Increased minimum from 48 to 100
    
    let img_resized = image::imageops::resize(
        &cropped,
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
    );
    
    // Create RGB tensor data in NCHW format and normalize to [-1, 1]
    // The recognition model expects 3 channels (RGB), not grayscale
    let mut data_vec = Vec::with_capacity(3 * target_height as usize * target_width as usize);
    
    // Fill in the data in NCHW format (all of channel 0, then all of channel 1, then channel 2)
    for c in 0..3 {
        for y in 0..target_height {
            for x in 0..target_width {
                let pixel = img_resized.get_pixel(x, y);
                let value = pixel[c as usize] as f32 / 255.0;
                // Normalize to [-1, 1]
                let normalized = (value - 0.5) / 0.5;
                data_vec.push(normalized);
            }
        }
    }
    
    // Create TensorData with shape [1, 3, H, W]
    let device = Default::default();
    let shape_vec = vec![1, 3, target_height as usize, target_width as usize];
    debug!("Creating recognition tensor with shape: {:?}, data length: {}", shape_vec, data_vec.len());
    let tensor_data = TensorData::new(data_vec, shape_vec);
    
    let tensor = Tensor::<B, 4>::from_data(tensor_data.convert::<f32>(), &device);
    debug!("Recognition tensor shape: {:?}", tensor.shape());
    tensor
}

/// Post-process recognition model output to extract text using CTC decoding
/// The recognition model outputs character probabilities for each time step
fn postprocess_recognition(output: Tensor<B, 3>, dictionary: &[String]) -> (String, f32) {
    debug!("Post-processing recognition output with CTC decoding");
    
    // Get the output shape: [batch, time_steps, num_classes]
    let output_data = output.to_data();
    let shape = output_data.shape.clone();
    
    // For batch size of 1, we extract [time_steps, num_classes]
    if shape[0] != 1 {
        debug!("Warning: Expected batch size 1, got {}", shape[0]);
        return (String::new(), 0.0);
    }
    
    let time_steps = shape[1];
    let num_classes = shape[2];
    
    // Convert tensor data to vector
    let values: Vec<f32> = output_data.as_slice::<f32>()
        .map(|s| s.to_vec())
        .unwrap_or_default();
    
    if values.is_empty() {
        debug!("Warning: Empty output tensor");
        return (String::new(), 0.0);
    }
    
    // CTC greedy decoding: take argmax at each time step
    let mut decoded_indices = Vec::new();
    let mut confidences = Vec::new();
    
    for t in 0..time_steps {
        let offset = t * num_classes;
        let mut max_idx = 0;
        let mut max_val = values[offset];
        
        for c in 1..num_classes {
            let val = values[offset + c];
            if val > max_val {
                max_val = val;
                max_idx = c;
            }
        }
        
        decoded_indices.push(max_idx);
        confidences.push(max_val);
    }
    
    // CTC decoding: remove blanks and consecutive duplicates
    // Typically, index 0 is the CTC blank token
    let blank_idx = 0;
    let mut result = String::new();
    let mut prev_idx = blank_idx;
    
    for &idx in &decoded_indices {
        if idx != blank_idx && idx != prev_idx {
            // Map index to character using dictionary
            // Account for blank token at index 0
            if idx > 0 && (idx - 1) < dictionary.len() {
                result.push_str(&dictionary[idx - 1]);
            }
        }
        prev_idx = idx;
    }
    
    // Calculate average confidence (excluding blank)
    let non_blank_confidences: Vec<f32> = decoded_indices.iter()
        .zip(confidences.iter())
        .filter_map(|(idx, conf)| {
            if *idx != blank_idx {
                Some(*conf)
            } else {
                None
            }
        })
        .collect();
    
    let avg_confidence = if non_blank_confidences.is_empty() {
        0.0
    } else {
        non_blank_confidences.iter().sum::<f32>() / non_blank_confidences.len() as f32
    };
    
    debug!("Decoded text: '{}' with confidence: {:.2}", result, avg_confidence);
    (result, avg_confidence)
}

/// Load the character dictionary
fn load_dictionary() -> Result<Vec<String>, String> {
    let dict_path = "models/en_dict.txt";
    let contents = std::fs::read_to_string(dict_path)
        .map_err(|e| format!("Failed to load dictionary: {}", e))?;
    
    Ok(contents.lines().map(|s| s.to_string()).collect())
}

/// Run OCR on an RGB image using Burn models
pub fn run_ocr_burn(image: RgbImage) -> Result<Vec<MyTextRegion>, String> {
    debug!("Starting Burn-based OCR");
    
    // Load dictionary
    let dictionary = load_dictionary()?;
    
    // Load the actual Burn models
    debug!("Loading detection model...");
    let detection_model = crate::burn_models::ppocrv4_mobile_det::Model::<B>::default();
    
    debug!("Loading recognition model...");
    let recognition_model = crate::burn_models::en_ppocrv4_mobile_rec::Model::<B>::default();
    
    // Step 1: Preprocess image for detection
    debug!("Preprocessing image for detection...");
    let detection_input = preprocess_image_for_detection(&image);
    
    // Step 2: Run detection model
    debug!("Running detection model...");
    let detection_output = detection_model.forward(detection_input);
    
    // Step 3: Post-process detection to get text regions
    debug!("Post-processing detection output...");
    let bboxes = postprocess_detection(detection_output);
    
    // Step 4: For each detected region, run recognition
    let mut results = Vec::new();
    
    for (idx, bbox) in bboxes.iter().enumerate() {
        let (x, y, w, h) = *bbox;
        debug!("Processing region {}: ({}, {}, {}, {})", idx, x, y, w, h);
        
        // Skip regions that are too small (safety check)
        if w < 32 || h < 16 {
            debug!("Skipping region {} - too small: {}x{}", idx, w, h);
            continue;
        }
        
        // Preprocess region for recognition
        let recognition_input = preprocess_region_for_recognition(&image, *bbox);
        
        // Run recognition model
        debug!("Running recognition model for region {}...", idx);
        let recognition_output = recognition_model.forward(recognition_input);
        
        // Post-process recognition
        let (text, confidence) = postprocess_recognition(recognition_output, &dictionary);
        
        if !text.is_empty() {
            debug!("Detected text: '{}' with confidence: {:.2}", text, confidence);
            results.push(MyTextRegion { text, confidence });
        }
    }
    
    debug!("Burn OCR completed with {} regions", results.len());
    Ok(results)
}
