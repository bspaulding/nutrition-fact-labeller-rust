# Burn Conversion Investigation Summary

## Issue
Convert nutrition-fact-labeller from ONNX Runtime to Burn deep learning framework.

## Investigation Results

### What Was Attempted
1. Added Burn and burn-import dependencies to Cargo.toml
2. Downloaded ONNX models for conversion
3. Created build script for ONNX-to-Burn conversion
4. Attempted compilation with multiple version combinations
5. **✅ Successfully resolved** by pinning candle-core to exact version

### Solution Found ✅
**The `burn-import` issue was successfully resolved by pinning `candle-core` to version `=0.9.1`:**

```toml
[dependencies]
candle-core = "=0.9.1"  # Exact version pin

[build-dependencies]
candle-core = "=0.9.1"  # Must pin in both sections
```

### Root Cause
- `burn-import` v0.20.1 was built against `candle-core` v0.9.1
- Cargo was resolving to `candle-core` v0.9.2 by default (latest compatible)
- Version 0.9.2 added new `DType` enum variants (I16, I32, F8E4M3, etc.)
- `burn-import` code didn't handle these new variants

### What Works Now
✅ `burn-import` compiles successfully  
✅ ONNX models convert to Burn at compile-time  
✅ Generated Rust code and weights (`.bpk` files)  
✅ Both detection and recognition models converted

### Generated Artifacts
```
ppocrv4_mobile_det.rs (117 KB) + ppocrv4_mobile_det.bpk (4.6 MB)
en_ppocrv4_mobile_rec.rs (113 KB) + en_ppocrv4_mobile_rec.bpk (7.3 MB)
```

## Conversion Complexity

With working burn-import, the conversion requires:

### Approach 1: Using burn-import ✅ (NOW WORKING)
- Convert ONNX models at compile-time ✅
- Implement inference pipeline wrappers
- Replace oar-ocr calls
- **Estimated**: ~1 week of focused work

### Approach 2: Manual Implementation (NO LONGER NEEDED)
- Would have required 2000+ lines of code, 2-4 weeks
- Not necessary now that burn-import works

## Documentation Provided

See [`BURN_CONVERSION.md`](./BURN_CONVERSION.md) for comprehensive documentation including:
- Working configuration with version pins
- Generated model details
- Detailed technical roadmap
- Implementation estimates (~1 week)
- Resources and references

## Recommendation

**Proceed with Burn conversion** using the working burn-import approach:

1. ✅ Models are already converted
2. Implement text detection pre/post-processing
3. Implement text recognition pre/post-processing
4. Replace oar-ocr usage incrementally
5. Test and validate results

## Next Steps

1. Implement detection pipeline (~2-3 days)
2. Implement recognition pipeline (~2-3 days)  
3. Integration and testing (~1-2 days)
4. Optional: Add orientation and unwarping models

## Key Learning

The workaround was simpler than expected:
- No code changes needed in burn-import
- Just an exact version pin: `candle-core = "=0.9.1"`
- Must be specified in both `[dependencies]` and `[build-dependencies]`
- Cargo's semver resolution needed explicit constraint

---

**Status**: Unblocked and ready for implementation  
**Date**: 2026-02-01  
**Models**: Successfully converted to Burn
