# CI Test Performance Optimization Options

## Current Situation

The Burn-based OCR implementation with NdArray CPU backend is functionally correct but slow:
- Detection model inference: ~2-3 minutes per image on 2-core CPU
- Recognition model: ~30-60 seconds per region
- Tests timeout in CI (5 minute default limit)

## Available Options

### Option 1: Use Larger CPU Runners (Requires Paid Plan)

**GitHub provides larger runners for Team and Enterprise plans:**

| Runner | CPU Cores | RAM | Cost per minute |
|--------|-----------|-----|-----------------|
| `ubuntu-latest` (current) | 2 | 7 GB | Free for public repos |
| `ubuntu-latest-4-core` | 4 | 16 GB | ~$0.016 |
| `ubuntu-latest-8-core` | 8 | 32 GB | ~$0.032 |
| `ubuntu-latest-16-core` | 16 | 64 GB | ~$0.064 |

**To use:** Change `runs-on: ubuntu-latest` to `runs-on: ubuntu-latest-4-core` in `.github/workflows/pr-tests.yml`

**Impact:** 2-4x improvement for CPU-bound operations, but still limited by single-threaded inference

**Cost estimate:** ~$0.32 per test run (20 minutes × $0.016/min)

### Option 2: GPU Runners (Requires Self-Hosting)

**GitHub does NOT provide GPU-enabled runners.** Options:

#### 2a. Self-Hosted Runner with GPU
- Set up your own GPU machine (on-prem or cloud)
- Install GitHub Actions runner agent
- Switch Burn backend to `burn-candle` with CUDA
- Expected speedup: 10-50x for CNN inference

**Setup steps:**
1. Provision GPU instance (e.g., AWS EC2 g4dn.xlarge with NVIDIA T4)
2. Install CUDA, NVIDIA drivers
3. Install GitHub Actions runner
4. Configure runner in repository settings
5. Update code to use burn-candle backend with CUDA

**Cost:** ~$0.50-1.00/hour for cloud GPU instances

#### 2b. Third-Party CI with GPU
Services that provide GPU runners:
- **CircleCI**: GPU executors available
- **BuildKite**: GPU agents available
- **GitLab CI**: GPU runners available
- **AWS CodeBuild**: GPU compute available

**Pros:** Managed infrastructure
**Cons:** Additional service integration, different pricing models

### Option 3: Optimize Current Implementation (✅ Implemented)

**No-cost optimizations:**

#### 3a. Use Release Builds (✅ Done)
Changed workflow to use `cargo test --release` instead of debug builds
- **Expected improvement:** 2-10x faster due to compiler optimizations
- **Status:** Implemented in `.github/workflows/pr-tests.yml`

#### 3b. Cache Improvements
Current caching covers:
- Cargo registry
- Cargo index  
- Build artifacts

**Could add:**
- Cache generated model files (.bpk) after first build
- Pre-build models in separate job

#### 3c. Reduce Test Scope
Options:
- Skip OCR tests in CI, mark as `#[ignore]` and run locally
- Use smaller test images
- Test with fewer test cases
- Split fast and slow tests

### Option 4: Switch to Faster Burn Backend

**Current:** `burn-ndarray` (pure Rust CPU implementation)

**Alternatives:**

#### 4a. burn-tch (PyTorch backend)
```toml
[dependencies]
burn = { version = "0.20.1", features = ["tch"] }
burn-tch = "0.20.1"
```

**Pros:**
- Optimized CPU inference (Intel MKL, OpenBLAS)
- 5-20x faster than ndarray on CPU
- Can use GPU if available

**Cons:**
- Requires libtorch installation
- Larger dependency footprint
- More complex build

#### 4b. burn-candle (Rust-native, optimized)
```toml
[dependencies]  
burn = { version = "0.20.1", features = ["candle"] }
burn-candle = "0.20.1"
```

**Pros:**
- Pure Rust (no C++ dependencies)
- Better CPU performance than ndarray
- GPU support with CUDA/Metal
- 3-10x faster on CPU

**Cons:**
- GPU requires CUDA/Metal setup
- Still slower than PyTorch on CPU

### Option 5: Hybrid Approach

**For CI:** Use simplified tests or mocked models
**For local development:** Full integration tests with real models
**For production:** Use appropriate backend based on deployment environment

## Recommendations

### Immediate (No Cost):
✅ **Done:** Use `cargo test --release` for optimizations
- **Next:** Add `#[ignore]` attribute to slow OCR tests
- Run full tests locally or in nightly CI builds

### Short Term (Moderate effort):
1. **Switch to burn-candle backend** for better CPU performance
2. **If budget allows:** Use `ubuntu-latest-4-core` runner ($0.32/test run)

### Long Term (Production ready):
1. **For cloud deployment:** Use burn-candle with GPU (AWS, GCP, Azure)
2. **For local inference:** Use burn-tch for optimized CPU performance
3. **Consider:** Self-hosted GPU runner if running many CI builds

## Implementation Examples

### Using Larger Runner (Paid Plan Required):
```yaml
# .github/workflows/pr-tests.yml
jobs:
  test:
    runs-on: ubuntu-latest-4-core  # Requires GitHub Team/Enterprise
```

### Skip Slow Tests in CI:
```rust
// In tests
#[test]
#[cfg_attr(not(feature = "slow_tests"), ignore)]
fn check_test_cases() {
    // OCR tests...
}
```

Then in CI:
```yaml
- name: Run fast tests
  run: cargo test --release --verbose
  
- name: Run slow tests (optional)
  run: cargo test --release --verbose --ignored
  if: github.event_name == 'schedule' # Run in nightly builds only
```

### Switch to burn-tch Backend:
```rust
// src/burn_ocr.rs
#[cfg(feature = "tch")]
type B = burn_tch::LibTorch;

#[cfg(not(feature = "tch"))]
type B = burn_ndarray::NdArray<f32>;
```

## Current Status

✅ **Implemented:** Release build optimization in CI workflow
📋 **Documented:** All available options with pros/cons/costs
⏭️ **Next Steps:** Choose approach based on budget and requirements

## Cost-Benefit Analysis

| Option | Cost | Setup Time | Performance Gain | Recommendation |
|--------|------|------------|------------------|----------------|
| Release builds | Free | 5 min ✅ | 2-10x | **Implemented** |
| 4-core runner | $0.32/run | 2 min | 2-4x | If budget allows |
| burn-tch backend | Free | 2-4 hours | 5-20x | **Recommended** |
| burn-candle backend | Free | 1-2 hours | 3-10x | Good alternative |
| Self-hosted GPU | $0.50-1/hr | 1-2 days | 10-50x | For high volume |
| Third-party GPU CI | Varies | 4-8 hours | 10-50x | For managed solution |

## References

- [GitHub Actions larger runners](https://docs.github.com/en/actions/using-github-hosted-runners/about-larger-runners)
- [Burn backends documentation](https://burn.dev/book/building-blocks/backend.html)
- [Self-hosted runners guide](https://docs.github.com/en/actions/hosting-your-own-runners)
