# Models Directory

This directory should contain the ONNX model files for PaddleOCR text detection and recognition.

## Required Files

1. **ppocrv4_mobile_det.onnx** - Text detection model (~4.6 MB)
2. **en_ppocrv4_mobile_rec.onnx** - Text recognition model (~7.3 MB)
3. **en_dict.txt** - Character dictionary for recognition (already present)

## Where to Get the Models

The models need to be downloaded from a PaddleOCR ONNX model repository. Options:

### Option 1: From oar-ocr package
The `oar-ocr` Rust crate includes these models. You can extract them from the crate or use the original source.

### Option 2: Convert from PaddleOCR
You can convert PaddleOCR models to ONNX format using paddle2onnx:
```bash
paddle2onnx --model_dir ./ch_PP-OCRv4_det_infer \
  --model_filename inference.pdmodel \
  --params_filename inference.pdiparams \
  --save_file ppocrv4_mobile_det.onnx
```

### Option 3: From Hugging Face (if available)
Some users host converted ONNX models on Hugging Face Model Hub.

## Current Status

⚠️ The model files in this directory are currently placeholder HTML files (from failed download attempts) and need to be replaced with actual ONNX files.

## To Build

Once you have the proper ONNX files:

1. Place them in this directory
2. Run `cargo build` - the build script will convert them to Burn format at compile time
3. The generated Burn models will be output to `target/*/build/nutrition-fact-labeller-*/out/generated/`

## Model Format

The ONNX files should:
- Be valid ONNX format (binary protobuf)
- Use opset version 11 or higher
- Include the `adaptive_avg_pool2d` operation (supported by NdArray backend)
- Match the input/output specifications expected by the preprocessing/postprocessing code
