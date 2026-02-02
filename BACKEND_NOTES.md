# Burn Backend Selection Notes

## Current Backend: LibTorch ✅

The project currently uses the **LibTorch (PyTorch)** backend for Burn inference, providing 5-20x better performance than NdArray.

## Backend Comparison and Limitations

### 1. LibTorch (burn-tch) Backend (Current) ✅
**Pros:**
- ✅ **5-20x faster** than NdArray on CPU
- ✅ Complete operation support (including `adaptive_avg_pool2d`)
- ✅ GPU support available (CUDA/Metal)
- ✅ Battle-tested PyTorch backend
- ✅ Expected inference time: ~2-5 seconds per image

**Cons:**
- ⚠️ Requires external **libtorch** C++ library installation
- ⚠️ More complex setup and deployment
- ⚠️ Larger dependencies

**Status:** **Currently used** - best performance, requires libtorch

### 2. NdArray Backend
**Pros:**
- ✅ Complete operation support (including `adaptive_avg_pool2d`)
- ✅ Pure Rust implementation
- ✅ No external dependencies
- ✅ Easy to set up and deploy

**Cons:**
- ⚠️ Much slower performance (~10-30 seconds per image in release)
- ⚠️ CPU only

**Status:** **Previously used** - works but slow, pure Rust

### 3. Candle Backend ❌
**Pros:**
- ✅ 3-10x faster than NdArray on CPU
- ✅ Pure Rust implementation
- ✅ GPU support available (CUDA/Metal)

**Cons:**
- ❌ **Does NOT support `adaptive_avg_pool2d` operation**
- ❌ Cannot run the PaddleOCR models which use this operation

**Status:** **Not compatible** with current ONNX models

## Why LibTorch?

The PaddleOCR models converted from ONNX use the `adaptive_avg_pool2d` operation, which is:
- ✅ Supported by LibTorch
- ✅ Supported by NdArray  
- ❌ NOT supported by Candle (as of v0.9.1)

LibTorch provides the best balance of:
1. **Performance**: 5-20x faster than NdArray
2. **Compatibility**: Supports all ONNX operations
3. **Maturity**: Well-tested PyTorch backend
4. **GPU Support**: Can leverage CUDA/Metal if available

The trade-off is requiring libtorch as an external dependency.

## LibTorch Installation

### Prerequisites

Download and install libtorch from: https://pytorch.org/get-started/locally/

**Linux/macOS (CPU):**
```bash
# Download libtorch
wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.1.0%2Bcpu.zip
unzip libtorch-cxx11-abi-shared-with-deps-2.1.0+cpu.zip

# Set environment variables
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH

# Add to ~/.bashrc or ~/.zshrc for persistence
echo 'export LIBTORCH=/path/to/libtorch' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH' >> ~/.bashrc
```

**Linux/macOS (CUDA/GPU):**
```bash
# Download CUDA version instead (requires NVIDIA GPU)
wget https://download.pytorch.org/libtorch/cu118/libtorch-cxx11-abi-shared-with-deps-2.1.0%2Bcu118.zip
unzip libtorch-cxx11-abi-shared-with-deps-2.1.0+cu118.zip

# Set environment variables
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH
```

**macOS (Alternative with Homebrew):**
```bash
# Install PyTorch via Homebrew
brew install pytorch

# Set LIBTORCH to homebrew installation
export LIBTORCH=$(brew --prefix pytorch)/lib
```

**Windows:**
1. Download libtorch from PyTorch website
2. Extract to a location (e.g., `C:\libtorch`)
3. Set environment variables:
   - `LIBTORCH=C:\libtorch`
   - Add `C:\libtorch\lib` to `PATH`

### Verification

After installation, verify with:
```bash
cargo build
```

If you see linking errors, check that:
1. `LIBTORCH` environment variable is set
2. `LD_LIBRARY_PATH` (Linux/macOS) or `PATH` (Windows) includes libtorch libraries
3. libtorch version is compatible (2.x recommended)

## Switching Back to NdArray

If you can't install libtorch or prefer pure Rust:

**1. Update Cargo.toml:**
```toml
[dependencies]
burn = { version = "0.20.1", features = ["ndarray"] }
burn-ndarray = "0.20.1"
# Remove: tch = "0.18"
```

**2. Update src/burn_ocr.rs:**
```rust
use burn::backend::ndarray::NdArray;
type B = NdArray<f32>;

// Change device initialization:
let device = Default::default();
```

**3. Rebuild:**
```bash
cargo clean
cargo build --release
```

Performance will be slower (~10-30 seconds per image) but no external dependencies needed.

## Performance Comparison

| Backend | Per-Image Inference | Setup Complexity | Dependencies |
|---------|---------------------|------------------|--------------|
| **LibTorch (current)** | **~2-5 seconds** | Medium | libtorch C++ library |
| NdArray | ~10-30 seconds | Easy | None (pure Rust) |
| Candle | N/A (incompatible) | Easy | None (pure Rust) |

## Conclusion

- **Current choice**: LibTorch for best performance
- **Trade-off**: Requires libtorch installation
- **Alternative**: NdArray if pure Rust is required (slower but works)
- **Not viable**: Candle (missing adaptive_avg_pool2d operation)
