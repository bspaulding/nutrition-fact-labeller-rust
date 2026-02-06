# Burn Backend Selection Notes

## Current Backend: WGPU

The project currently uses the **WGPU** backend for Burn inference, providing GPU acceleration across multiple platforms.

## Backend Comparison and Limitations

### 1. WGPU Backend (Current) ✅
**Pros:**
- ✅ GPU acceleration via WebGPU API
- ✅ Cross-platform (Windows, Linux, macOS, Web)
- ✅ Pure Rust implementation
- ✅ No external C++ dependencies (unlike LibTorch)
- ✅ Significantly faster than NdArray CPU backend
- ✅ Complete operation support (including `adaptive_avg_pool2d`)

**Cons:**
- ⚠️ Requires GPU/graphics drivers for best performance
- ⚠️ Fallback to CPU if GPU unavailable

**Status:** **Currently used** - provides GPU acceleration with no external dependencies

### 2. NdArray Backend ✅
**Pros:**
- ✅ Complete operation support (including `adaptive_avg_pool2d`)
- ✅ Pure Rust implementation
- ✅ No external dependencies
- ✅ Easy to set up and deploy

**Cons:**
- ⚠️ Slower performance (~2-3 minutes per image in debug, ~10-30 seconds in release)
- ⚠️ CPU only

**Status:** **Alternative option** - works but slower than WGPU

### 3. Candle Backend ❌
**Pros:**
- ✅ 3-10x faster than NdArray on CPU
- ✅ Pure Rust implementation
- ✅ GPU support available (CUDA/Metal)

**Cons:**
- ❌ **Does NOT support `adaptive_avg_pool2d` operation**
- ❌ Cannot run the PaddleOCR models which use this operation

**Status:** **Not compatible** with current ONNX models

### 4. LibTorch (burn-tch) Backend ⚠️
**Pros:**
- ✅ 5-20x faster than NdArray on CPU
- ✅ Complete operation support (including `adaptive_avg_pool2d`)
- ✅ GPU support
- ✅ Battle-tested PyTorch backend

**Cons:**
- ❌ Requires external **libtorch** C++ library installation
- ⚠️ More complex setup and deployment
- ⚠️ Larger dependencies

**Status:** **Viable but not currently used** - requires libtorch installation

## Why WGPU?

The PaddleOCR models converted from ONNX use the `adaptive_avg_pool2d` operation, which is:
- ✅ Supported by WGPU
- ✅ Supported by NdArray
- ❌ **NOT supported by Candle** (as of v0.9.1)
- ✅ Supported by LibTorch

**WGPU provides the best balance:**
- GPU acceleration for significantly better performance than NdArray
- Pure Rust with no external C++ dependencies (unlike LibTorch)
- Cross-platform support (Windows, Linux, macOS, Web)
- Complete operation support including `adaptive_avg_pool2d`

WGPU gives us the performance benefits of GPU acceleration while maintaining the ease of deployment of a pure Rust solution.

## Performance Optimization Options

### Option 1: Use Release Builds (✅ Implemented)
```bash
cargo test --release
cargo build --release
```
**Improvement:** 2-10x faster  
**Cost:** None  
**Status:** Already implemented in CI

### Option 2: Switch to LibTorch Backend
**Requirements:**
1. Install libtorch on the system
2. Update Cargo.toml to use burn-tch
3. Update device initialization in code

**Improvement:** 5-20x faster than NdArray  
**Cost:** External dependency, more complex deployment  
**Status:** Not implemented - requires infrastructure changes

### Option 3: Optimize Models
- Use smaller/quantized models
- Reduce input resolution
- Cache model loading

**Improvement:** 2-5x faster  
**Cost:** May reduce accuracy  
**Status:** Not explored

### Option 4: Use GPU (with LibTorch or Candle if models are compatible)
**Requirements:**
- LibTorch backend with CUDA support, OR
- Different models that don't use adaptive_avg_pool2d for Candle

**Improvement:** 10-50x faster  
**Cost:** Requires GPU hardware, more complex setup  
**Status:** Not feasible with current Candle + these models

## Recommendations

**For Development:**
- Use release builds (`cargo test --release`)
- Accept slower inference times
- Current setup works correctly

**For Production:**
- Consider switching to LibTorch backend if deployment can handle the dependency
- OR optimize for smaller models/lower resolution
- OR use a faster machine/more CPU cores

**For CI:**
- Use release builds (already implemented)
- Consider larger GitHub runners if budget allows (4/8/16 cores)
- Current setup should work within timeout limits with release builds

## How to Switch Backends

### To use NdArray (CPU-only fallback):

1. Update `Cargo.toml`:
```toml
[dependencies]
burn = { version = "0.20.1", features = ["ndarray"] }
burn-ndarray = "0.20.1"
```

2. Update `src/burn_ocr.rs`:
```rust
use burn::backend::ndarray::NdArray;
type B = NdArray<f32>;
// Replace device initialization:
let device = Default::default();
```

### To use LibTorch (if libtorch is installed):

1. Update `Cargo.toml`:
```toml
[dependencies]
burn = { version = "0.20.1", features = ["tch"] }
burn-tch = "0.20.1"
```

2. Update `src/burn_ocr.rs`:
```rust
use burn::backend::libtorch::{LibTorch, LibTorchDevice};
type B = LibTorch;
// Replace device initialization:
let device = LibTorchDevice::Cpu;  // or LibTorchDevice::Cuda(0) for GPU
```

3. Install libtorch on your system

### To use Candle (not recommended with current models):
Would require different ONNX models that don't use `adaptive_avg_pool2d`.

## Summary

We use WGPU because it provides:
1. GPU acceleration for significantly better performance
2. Complete operation support (including `adaptive_avg_pool2d`)
3. Pure Rust with no external C++ dependencies
4. Cross-platform compatibility (Windows, Linux, macOS, Web)
5. Easy deployment - no need to install external libraries like libtorch

WGPU gives us the best of both worlds: high performance GPU acceleration AND easy deployment with no external dependencies.
