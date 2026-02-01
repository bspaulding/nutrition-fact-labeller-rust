# Burn Conversion Investigation Summary

## Issue
Convert nutrition-fact-labeller from ONNX Runtime to Burn deep learning framework.

## Investigation Results

### What Was Attempted
1. Added Burn and burn-import dependencies to Cargo.toml
2. Downloaded ONNX models for conversion
3. Created build script for ONNX-to-Burn conversion
4. Attempted compilation with multiple version combinations

### Critical Blocker Found
**The `burn-import` crate (v0.20.1) does not currently compile** due to incompatibility with `candle-core` v0.9.2:

```
error[E0004]: non-exhaustive patterns: `candle_core::DType::I16`, 
`candle_core::DType::I32`, `candle_core::DType::F8E4M3` and 4 more not covered
```

This is an **upstream issue** in the Burn ecosystem that must be resolved before conversion can proceed.

### Workarounds Attempted
- ❌ Version pinning of candle-core (transitive dependencies still pulled newer version)
- ❌ Using older burn versions (not available/compatible)
- ❌ Manual dependency resolution (conflicts throughout dependency tree)

## Conversion Complexity

Even with working burn-import, this conversion would require:

### Approach 1: Using burn-import
- Convert ONNX models at compile-time
- Replace inference calls
- **Estimated**: 1-2 days (blocked)

### Approach 2: Manual Implementation
- Implement text detection pipeline
- Implement text recognition pipeline  
- Implement post-processing (NMS, CTC decode, etc.)
- **Estimated**: 2000+ lines of code, 2-4 weeks

## Documentation Provided

See [`BURN_CONVERSION.md`](./BURN_CONVERSION.md) for comprehensive documentation including:
- Detailed technical analysis
- Three conversion approaches with trade-offs
- Complete roadmap for when unblocked
- Model specifications and requirements
- Resources and references

## Recommendation

**Wait for upstream fix** in burn-import, then proceed with Approach 1 (using burn-import).

The project currently remains functional using ONNX Runtime via the `oar-ocr` crate.

## Next Steps

1. Monitor https://github.com/tracel-ai/burn-onnx for updates
2. Watch for new burn-import releases
3. Consider opening issue on burn-onnx repository
4. Alternatively, contribute fix to burn-import

---

**Status**: Investigation complete, conversion blocked by upstream issue  
**Date**: 2026-02-01
