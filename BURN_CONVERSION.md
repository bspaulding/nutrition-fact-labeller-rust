# Converting to Burn Deep Learning Framework

## Summary

This document outlines the work required to convert this project from using ONNX Runtime (via `oar-ocr`) to the Burn deep learning framework.

## Current Status: UNBLOCKED ✅

**Solution Found**: Pinning `candle-core` to version `=0.9.1` successfully resolves the compilation issues with `burn-import` v0.20.1.

### Working Configuration
```toml
[dependencies]
burn = { version = "0.20.1", features = ["ndarray"] }
burn-ndarray = "0.20.1"
burn-import = { version = "0.20.1", features = ["onnx"] }
candle-core = "=0.9.1"  # Exact version pin is critical

[build-dependencies]
burn-import = { version = "0.20.1", features = ["onnx"] }
candle-core = "=0.9.1"  # Must pin in both sections
```

### What Works Now
✅ `burn-import` compiles successfully with `candle-core` v0.9.1  
✅ ONNX models convert to Burn at compile-time  
✅ Generated Rust code for both detection and recognition models  
✅ Models include weights in `.bpk` (BurnPack) format  
✅ Type-safe inference API generated

### Generated Artifacts
- `ppocrv4_mobile_det.rs` (117 KB) + `ppocrv4_mobile_det.bpk` (4.6 MB)
- `en_ppocrv4_mobile_rec.rs` (113 KB) + `en_ppocrv4_mobile_rec.bpk` (7.3 MB)

## Background

### Current Implementation
- Uses `oar-ocr` v0.2.2 library
- `oar-ocr` uses ONNX Runtime (`ort` crate) for model inference
- Supports PaddleOCR models for:
  - Text detection (DBNet-based)
  - Text recognition (CRNN-based)
  - Document orientation classification
  - Document unwarping
  - Text line orientation

### Why Convert to Burn?
- **Pure Rust**: Burn is a native Rust framework, reducing C/C++ dependencies
- **Flexibility**: More control over model execution and optimization
- **Modern**: Leverages Rust's type system for safe tensor operations
- **Cross-platform**: Better support for various backends (CPU, CUDA, WebGPU, etc.)

## Conversion Approaches

### Approach 1: Using burn-import (NOW WORKING)

This is the most straightforward approach and is now unblocked:

1. ✅ Use `burn-import` to convert ONNX models to Burn at compile-time
2. ✅ Generated Rust code can run on any Burn backend
3. 🔄 Replace `oar-ocr` inference calls with Burn inference (in progress)

**Status**: Models successfully converted, inference pipeline implementation needed

**Remaining Work**:
- Implement text detection pre/post-processing
- Implement text recognition pre/post-processing  
- Integrate with existing application code

### Approach 2: Manual Implementation (NO LONGER NEEDED)

This approach is no longer necessary since burn-import works with the version pin.

### Approach 3: Hybrid Approach (CURRENT STATE)

Current implementation demonstrates both:
- Burn models successfully compile and are available
- oar-ocr continues to work for functionality during migration

## Technical Details

### Models Used
1. **ppocrv4_mobile_det.onnx** (4.7 MB)
   - Text detection model
   - Input: RGB image, variable size
   - Output: Probability map
   - ✅ Successfully converted to Burn

2. **en_ppocrv4_mobile_rec.onnx** (7.5 MB)
   - Text recognition model
   - Input: Normalized text region
   - Output: Character probabilities
   - ✅ Successfully converted to Burn

3. **pplcnet_x1_0_doc_ori.onnx**
   - Document orientation (0°, 90°, 180°, 270°)
   - 🔄 Conversion pending

4. **pplcnet_x1_0_textline_ori.onnx**
   - Text line orientation (0° or 180°)
   - 🔄 Conversion pending

5. **uvdoc.onnx**
   - Document unwarping/rectification
   - 🔄 Conversion pending

### Burn Backend Options
- **NdArray**: CPU-only, no external dependencies (currently configured)
- **WGPU**: GPU acceleration via WebGPU
- **CUDA**: NVIDIA GPU acceleration
- **LibTorch**: PyTorch backend (requires LibTorch C++ library)
- **Candle**: HuggingFace's Candle backend

## Roadmap

### Phase 1: Environment Setup ✅
- [x] Identify working burn-import version combination
- [x] Set up build system for ONNX to Burn conversion
- [x] Verify model conversion works correctly
- [x] Generate Burn models at compile-time

### Phase 2: Core Implementation (In Progress)
- [ ] Implement text detection pre-processing
- [ ] Implement text detection post-processing (binarization, contours, NMS)
- [ ] Implement text recognition pre-processing
- [ ] Implement text recognition post-processing (CTC decode)
- [ ] Test with sample images

### Phase 3: Optional Features
- [ ] Add orientation detection models
- [ ] Add document unwarping model
- [ ] Optimize performance

### Phase 4: Integration
- [ ] Replace `oar-ocr` calls in main.rs
- [ ] Update tests
- [ ] Update Docker container
- [ ] Performance benchmarking

## Implementation Estimate

With burn-import working:
- **Text Detection Pipeline**: ~2-3 days
- **Text Recognition Pipeline**: ~2-3 days
- **Integration & Testing**: ~1-2 days
- **Total**: ~1 week of focused development

## Key Learnings

### The Version Pin Solution
The issue was that `burn-import` v0.20.1 was built against `candle-core` v0.9.1, but Cargo was resolving to v0.9.2 which added new `DType` variants that burn-import didn't handle. The solution:

```toml
candle-core = "=0.9.1"  # Exact version pin with "="
```

This must be specified in **both** `[dependencies]` and `[build-dependencies]` sections.

### Why It Works
- Burn workspace specifies `candle-core = { version = "0.9.1" }`
- Without explicit pin, Cargo resolves to latest compatible (0.9.2)
- With `=0.9.1` pin, Cargo uses exact version across all dependencies
- burn-import code matches the DType enum from 0.9.1

## Resources

- Burn Framework: https://github.com/tracel-ai/burn
- Burn ONNX Import: https://github.com/tracel-ai/burn-onnx
- Burn Book: https://burn.dev/book/
- PaddleOCR: https://github.com/PaddlePaddle/PaddleOCR
- oar-ocr: https://github.com/GreatV/oar-ocr

## Contributing

To continue the conversion:

1. Implement detection pre-processing (image normalization, resizing)
2. Implement detection post-processing (threshold, contours, NMS)
3. Implement recognition pre-processing (text region extraction)
4. Implement recognition post-processing (CTC decode, character mapping)
5. Test inference with sample images
6. Replace oar-ocr usage in main application

---

Last Updated: 2026-02-01  
Status: Unblocked - Models converted successfully with candle-core v0.9.1 pin

