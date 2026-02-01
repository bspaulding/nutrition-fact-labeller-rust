# Converting to Burn Deep Learning Framework

## Summary

This document outlines the work required to convert this project from using ONNX Runtime (via `oar-ocr`) to the Burn deep learning framework.

## Current Status: BLOCKED

**Critical Blocker**: The `burn-import` crate (v0.20.1) does not currently compile due to incompatibilities with `candle-core` v0.9.2. The burn-import crate has not been updated to handle newer DType variants added to candle-core.

### Compilation Error
```
error[E0004]: non-exhaustive patterns: `candle_core::DType::I16`, `candle_core::DType::I32`, 
`candle_core::DType::F8E4M3` and 4 more not covered
  --> burn-import-0.20.1/src/common/candle.rs:62:15
```

This is an upstream issue in the Burn ecosystem that needs to be resolved before this conversion can proceed.

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

### Approach 1: Using burn-import (BLOCKED)

This is the most straightforward approach but is currently blocked:

1. Use `burn-import` to convert ONNX models to Burn at compile-time
2. Generated Rust code can run on any Burn backend
3. Replace `oar-ocr` inference calls with Burn inference

**Status**: Cannot proceed due to burn-import compilation issues

**Prerequisites**:
- burn-import must be fixed to work with latest candle-core
- OR use older, compatible versions (requires significant dependency downgrades)

### Approach 2: Manual Implementation (LARGE EFFORT)

Implement OCR pipeline from scratch using Burn's low-level APIs:

**Required Components**:

1. **Model Loading** (~200 LOC)
   - Load ONNX weights manually
   - Convert to Burn tensor format
   - Handle different data types and shapes

2. **Text Detection** (~500 LOC)
   - Image preprocessing (normalization, resizing)
   - DBNet model inference
   - Post-processing (binarization, contour detection)
   - Non-Maximum Suppression (NMS)
   - Polygon to bounding box conversion

3. **Text Recognition** (~400 LOC)
   - Text region extraction and normalization
   - CRNN model inference
   - CTC decoding
   - Character dictionary lookup

4. **Optional Models** (~300 LOC each)
   - Document orientation classification
   - Text line orientation
   - Document unwarping

**Estimated Total**: 2000+ lines of code, 2-4 weeks of development

**Challenges**:
- Understanding PaddleOCR model architectures
- Implementing complex post-processing algorithms
- Testing and debugging inference pipeline
- Performance optimization

### Approach 3: Hybrid Approach

Keep ONNX Runtime but use Burn for other parts:
- Use Burn for custom layers or preprocessing
- Keep ONNX Runtime for model inference
- Less disruption, incremental migration

## Technical Details

### Models Used
1. **ppocrv4_mobile_det.onnx** (4.7 MB)
   - Text detection model
   - Input: RGB image, variable size
   - Output: Probability map

2. **en_ppocrv4_mobile_rec.onnx** (7.5 MB)
   - Text recognition model
   - Input: Normalized text region
   - Output: Character probabilities

3. **pplcnet_x1_0_doc_ori.onnx**
   - Document orientation (0°, 90°, 180°, 270°)

4. **pplcnet_x1_0_textline_ori.onnx**
   - Text line orientation (0° or 180°)

5. **uvdoc.onnx**
   - Document unwarping/rectification

### Burn Backend Options
- **NdArray**: CPU-only, no external dependencies
- **WGPU**: GPU acceleration via WebGPU
- **CUDA**: NVIDIA GPU acceleration
- **LibTorch**: PyTorch backend (requires LibTorch C++ library)
- **Candle**: HuggingFace's Candle backend

## Roadmap When Unblocked

### Phase 1: Environment Setup
- [ ] Wait for burn-import fix OR identify working version combination
- [ ] Set up build system for ONNX to Burn conversion
- [ ] Verify model conversion works correctly

### Phase 2: Core Implementation
- [ ] Implement text detection pipeline
- [ ] Implement text recognition pipeline
- [ ] Test with sample images

### Phase 3: Optional Features
- [ ] Add orientation detection
- [ ] Add document unwarping
- [ ] Optimize performance

### Phase 4: Integration
- [ ] Replace `oar-ocr` calls in main.rs
- [ ] Update tests
- [ ] Update Docker container
- [ ] Performance benchmarking

## Dependencies When Unblocked

```toml
[dependencies]
burn = { version = "0.20", features = ["ndarray"] }
burn-ndarray = "0.20"

[build-dependencies]
burn-import = { version = "0.20", features = ["onnx"] }
```

## Alternative: Wait for burn-onnx Repository

The Burn team maintains a separate `burn-onnx` repository (https://github.com/tracel-ai/burn-onnx) that may have more recent fixes. However, as of the time of this writing, the published `burn-import` crate on crates.io still has the compilation issues.

## Recommendations

1. **Short-term**: Keep using `oar-ocr` until burn-import is fixed
2. **Medium-term**: Monitor burn-import releases for fixes
3. **Long-term**: Consider contributing a fix to burn-import
4. **Alternative**: Implement manual ONNX weight loading if burn-import remains broken

## Resources

- Burn Framework: https://github.com/tracel-ai/burn
- Burn ONNX Import: https://github.com/tracel-ai/burn-onnx
- Burn Book: https://burn.dev/book/
- PaddleOCR: https://github.com/PaddlePaddle/PaddleOCR
- oar-ocr: https://github.com/GreatV/oar-ocr

## Contributing

If you want to help unblock this conversion:

1. Check burn-import repository for existing issues
2. Test with different version combinations
3. Consider contributing a fix to burn-import
4. OR help implement manual ONNX loading for Burn

---

Last Updated: 2026-02-01
Status: Blocked by upstream burn-import compilation issues
